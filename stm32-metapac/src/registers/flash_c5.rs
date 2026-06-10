
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Flash",
            extends: None,
            description: Some(
                "FLASH register block.",
            ),
            items: &[
                BlockItem {
                    name: "acr",
                    description: Some(
                        "Each register is assigned a offset address and a reset value. In case of registers representing option byte value, the reset value is determined by the OBL process. In case of success the reset value is loaded from OB. In case of OBL failure, a highly restrictive default value is set.FLASH access control register.",
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
                    name: "keyr",
                    description: Some(
                        "FLASH key register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Keyr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "optkeyr",
                    description: Some(
                        "FLASH option key register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Optkeyr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "opsr",
                    description: Some(
                        "FLASH operation status register.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Opsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "optcr",
                    description: Some(
                        "FLASH option control register.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Optcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sr",
                    description: Some(
                        "FLASH status register.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Sr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr",
                    description: Some(
                        "FLASH control register.",
                    ),
                    array: None,
                    byte_offset: 0x28,
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
                    name: "ccr",
                    description: Some(
                        "FLASH clear control register.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "privcfgr",
                    description: Some(
                        "FLASH privilege configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x3c,
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
                    name: "hdpextr",
                    description: Some(
                        "FLASH HDP extension register.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Hdpextr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "optsr_cur",
                    description: Some(
                        "FLASH option status register.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "OptsrCur",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "optsr_prg",
                    description: Some(
                        "FLASH option status register.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "OptsrPrg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "optsr2_cur",
                    description: Some(
                        "FLASH option status register 2.",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Optsr2Cur",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "optsr2_prg",
                    description: Some(
                        "FLASH option status register 2.",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Optsr2Prg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bootr_cur",
                    description: Some(
                        "FLASH unique boot entry register.",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "BootrCur",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bootr_prg",
                    description: Some(
                        "FLASH unique boot entry address.",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "BootrPrg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "otpblr_cur",
                    description: Some(
                        "FLASH OTP block lock.",
                    ),
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "OtpblrCur",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "otpblr_prg",
                    description: Some(
                        "FLASH OTP block lock.",
                    ),
                    array: None,
                    byte_offset: 0x94,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "OtpblrPrg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bl_com_cfg_cur",
                    description: Some(
                        "FLASH Bootloader interface selection.",
                    ),
                    array: None,
                    byte_offset: 0x98,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "BlComCfgCur",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bl_com_cfg_prg",
                    description: Some(
                        "FLASH Bootloader interface selection.",
                    ),
                    array: None,
                    byte_offset: 0x9c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "BlComCfgPrg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "oemkeyr1_prg",
                    description: Some(
                        "FLASH OEM Key register 1.",
                    ),
                    array: None,
                    byte_offset: 0xa4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Oemkeyr1Prg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "oemkeyr2_prg",
                    description: Some(
                        "FLASH OEM Key register 2.",
                    ),
                    array: None,
                    byte_offset: 0xac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Oemkeyr2Prg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "oemkeyr3_prg",
                    description: Some(
                        "FLASH OEM Key register 3.",
                    ),
                    array: None,
                    byte_offset: 0xb4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Oemkeyr3Prg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "oemkeyr4_prg",
                    description: Some(
                        "FLASH OEM Key register 4.",
                    ),
                    array: None,
                    byte_offset: 0xbc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Oemkeyr4Prg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bskeyr_prg",
                    description: Some(
                        "FLASH Boundary Scan key register.",
                    ),
                    array: None,
                    byte_offset: 0xc4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "BskeyrPrg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wrp1r_cur",
                    description: Some(
                        "FLASH write page protection for bank1.",
                    ),
                    array: None,
                    byte_offset: 0xe8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Wrp1rCur",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wrp1r_prg",
                    description: Some(
                        "FLASH write page protection for bank1.",
                    ),
                    array: None,
                    byte_offset: 0xec,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wrp1rPrg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hdp1r_cur",
                    description: Some(
                        "FLASH HDP bank1 register.",
                    ),
                    array: None,
                    byte_offset: 0xf8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Hdp1rCur",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hdp1r_prg",
                    description: Some(
                        "FLASH HDP bank1 register.",
                    ),
                    array: None,
                    byte_offset: 0xfc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Hdp1rPrg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecccorr",
                    description: Some(
                        "FLASH ECC correction register.",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ecccorr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eccdetr",
                    description: Some(
                        "FLASH ECC detection register.",
                    ),
                    array: None,
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Eccdetr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eccdr",
                    description: Some(
                        "FLASH ECC data.",
                    ),
                    array: None,
                    byte_offset: 0x108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Eccdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wrp2r_cur",
                    description: Some(
                        "FLASH write page protection for bank2.",
                    ),
                    array: None,
                    byte_offset: 0x1e8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Wrp2rCur",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wrp2r_prg",
                    description: Some(
                        "FLASH write page protection for bank2.",
                    ),
                    array: None,
                    byte_offset: 0x1ec,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wrp2rPrg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hdp2r_cur",
                    description: Some(
                        "FLASH HDP bank2 register.",
                    ),
                    array: None,
                    byte_offset: 0x1f8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Hdp2rCur",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hdp2r_prg",
                    description: Some(
                        "FLASH HDP bank2 register.",
                    ),
                    array: None,
                    byte_offset: 0x1fc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Hdp2rPrg",
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
                "Each register is assigned a offset address and a reset value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "latency",
                    description: Some(
                        "Read latency.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Latency",
                    ),
                },
                Field {
                    name: "wrhighfreq",
                    description: Some(
                        "FLASH signal delay.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "prften",
                    description: Some(
                        "Prefetch enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Prften",
                    ),
                },
                Field {
                    name: "empty",
                    description: Some(
                        "Main Flash memory area empty (not reset by system reset).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Empty",
                    ),
                },
            ],
        },
        FieldSet {
            name: "BlComCfgCur",
            extends: None,
            description: Some(
                "FLASH Bootloader interface selection.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bl_com_cfg",
                    description: Some(
                        "Bootloader interface selection/configuration.",
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
            name: "BlComCfgPrg",
            extends: None,
            description: Some(
                "FLASH Bootloader interface selection.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bl_com_cfg",
                    description: Some(
                        "Bootloader interface selection/configuration.",
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
            name: "BootrCur",
            extends: None,
            description: Some(
                "FLASH unique boot entry register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "boot_lock",
                    description: Some(
                        "A field locking the values of BOOT0, BOOT_SEL, SWAP_BANK, and BOOTADD option settings.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: Some(
                        "BootrCurBootLock",
                    ),
                },
                Field {
                    name: "bootadd",
                    description: Some(
                        "unique boot entry address.",
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
            name: "BootrPrg",
            extends: None,
            description: Some(
                "FLASH unique boot entry address.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "boot_lock",
                    description: Some(
                        "A field locking the values of BOOT0, BOOT_SEL, SWAP_BANK, and BOOTADD option settings.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: Some(
                        "BootrPrgBootLock",
                    ),
                },
                Field {
                    name: "bootadd",
                    description: Some(
                        "unique boot entry address.",
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
            name: "BskeyrPrg",
            extends: None,
            description: Some(
                "FLASH Boundary Scan key register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bskey",
                    description: Some(
                        "Boundary Scan KEY.",
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
            name: "Ccr",
            extends: None,
            description: Some(
                "FLASH clear control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "clr_eop",
                    description: Some(
                        "EOP flag clear bit.",
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
                    name: "clr_wrperr",
                    description: Some(
                        "WRPERR flag clear bit.",
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
                    name: "clr_pgserr",
                    description: Some(
                        "PGSERR flag clear bit.",
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
                    name: "clr_strberr",
                    description: Some(
                        "STRBERR flag clear bit.",
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
                    name: "clr_incerr",
                    description: Some(
                        "INCERR flag clear bit.",
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
                    name: "clr_optchangeerr",
                    description: Some(
                        "Clear the flag corresponding flag in SR by writing this bit.",
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
            name: "Cr",
            extends: None,
            description: Some(
                "FLASH control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lock",
                    description: Some(
                        "configuration lock bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Lock",
                    ),
                },
                Field {
                    name: "pg",
                    description: Some(
                        "programming control bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pg",
                    ),
                },
                Field {
                    name: "per",
                    description: Some(
                        "page erase request.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Per",
                    ),
                },
                Field {
                    name: "ser",
                    description: Some(
                        "sector erase request.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ser",
                    ),
                },
                Field {
                    name: "ber",
                    description: Some(
                        "Bank erase request.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ber",
                    ),
                },
                Field {
                    name: "fw",
                    description: Some(
                        "write forcing control bit.",
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
                    name: "strt",
                    description: Some(
                        "erase start control bit.",
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
                    name: "pnb",
                    description: Some(
                        "page erase selection number.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: Some(
                        "Pnb",
                    ),
                },
                Field {
                    name: "snb",
                    description: Some(
                        "sector erase selection number.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: Some(
                        "Snb",
                    ),
                },
                Field {
                    name: "mer",
                    description: Some(
                        "Mass erase request.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Mer",
                    ),
                },
                Field {
                    name: "eopie",
                    description: Some(
                        "end of operation interrupt control bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Eopie",
                    ),
                },
                Field {
                    name: "wrperrie",
                    description: Some(
                        "write protection error interrupt enable bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Wrperrie",
                    ),
                },
                Field {
                    name: "pgserrie",
                    description: Some(
                        "programming sequence error interrupt enable bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pgserrie",
                    ),
                },
                Field {
                    name: "strberrie",
                    description: Some(
                        "strobe error interrupt enable bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Strberrie",
                    ),
                },
                Field {
                    name: "incerrie",
                    description: Some(
                        "inconsistency error interrupt enable bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Incerrie",
                    ),
                },
                Field {
                    name: "optchangeerrie",
                    description: Some(
                        "Option-byte change error interrupt enable bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Optchangeerrie",
                    ),
                },
                Field {
                    name: "edatasel",
                    description: Some(
                        "EDATA erase selector bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Edatasel",
                    ),
                },
                Field {
                    name: "bksel",
                    description: Some(
                        "Bank selector bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Bksel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Ecccorr",
            extends: None,
            description: Some(
                "FLASH ECC correction register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addr_ecc",
                    description: Some(
                        "ECC error address.",
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
                    name: "data_ecc",
                    description: Some(
                        "ECC fail for corrected ECC error in flash data area.",
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
                    name: "edata_ecc",
                    description: Some(
                        "ECC fail for corrected ECC error in flash data area.",
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
                    name: "bk_ecc",
                    description: Some(
                        "ECC bank flag for corrected ECC error.",
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
                    name: "sysf_ecc",
                    description: Some(
                        "ECC flag for corrected ECC error in system FLASH.",
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
                    name: "otp_ecc",
                    description: Some(
                        "OTP ECC error bit.",
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
                    name: "ecccie",
                    description: Some(
                        "ECC single correction error interrupt enable bit When ECCCIE bit is set to 1, an interrupt is generated when an ECC single correction error occurs during a read operation.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ecccie",
                    ),
                },
                Field {
                    name: "eccc",
                    description: Some(
                        "ECC correction.",
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
            ],
        },
        FieldSet {
            name: "Eccdetr",
            extends: None,
            description: Some(
                "FLASH ECC detection register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addr_ecc",
                    description: Some(
                        "ECC error address.",
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
                    name: "data_ecc",
                    description: Some(
                        "ECC fail for double ECC error in flash data area.",
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
                    name: "edata_ecc",
                    description: Some(
                        "ECC fail for double ECC error in flash data area.",
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
                    name: "bk_ecc",
                    description: Some(
                        "ECC fail bank for double ECC Error.",
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
                    name: "sysf_ecc",
                    description: Some(
                        "ECC fail for double ECC error in system flash memory.",
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
                    name: "otp_ecc",
                    description: Some(
                        "OTP ECC error bit.",
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
                    name: "eccd",
                    description: Some(
                        "ECC detection set by hardware when two ECC error has been detected.",
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
            name: "Eccdr",
            extends: None,
            description: Some(
                "FLASH ECC data.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data_ecc",
                    description: Some(
                        "ECC error data.",
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
                    name: "data_addr_ecc",
                    description: Some(
                        "DATA ECC error address.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "DataAddrEcc",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Hdp1rCur",
            extends: None,
            description: Some(
                "FLASH HDP bank1 register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hdp1_strt",
                    description: Some(
                        "Bank 1 HDPL barrier start set in number of 8Kbytes pages.",
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
                    name: "hdp1_end",
                    description: Some(
                        "Bank 1 HDPL barrier end set in number of 8Kbytes pages.",
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
            name: "Hdp1rPrg",
            extends: None,
            description: Some(
                "FLASH HDP bank1 register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hdp1_strt",
                    description: Some(
                        "Bank 1 HDPL barrier start set in number of 8Kbytes pages.",
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
                    name: "hdp1_end",
                    description: Some(
                        "Bank 1 HDPL barrier end set in number of 8Kbytes pages.",
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
            name: "Hdp2rCur",
            extends: None,
            description: Some(
                "FLASH HDP bank2 register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hdp2_strt",
                    description: Some(
                        "Bank 2 HDPL barrier start set in number of 8Kbytes pages.",
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
                    name: "hdp2_end",
                    description: Some(
                        "Bank 2 HDPL barrier end set in number of 8Kbytes pages.",
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
            name: "Hdp2rPrg",
            extends: None,
            description: Some(
                "FLASH HDP bank2 register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hdp2_strt",
                    description: Some(
                        "Bank 2 HDPL barrier start set in number of 8Kbytes pages.",
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
                    name: "hdp2_end",
                    description: Some(
                        "Bank 2 HDPL barrier end set in number of 8Kbytes pages.",
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
            name: "Hdpextr",
            extends: None,
            description: Some(
                "FLASH HDP extension register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hdp1_ext",
                    description: Some(
                        "HDP area extension in 8Kbytes pages in bank1. Extension is added after the HDP1_END page (included).",
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
                    name: "hdp2_ext",
                    description: Some(
                        "HDP area extension in 8Kbytes pages in bank2. Extension is added after the HDP2_END page (included).",
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
            name: "Keyr",
            extends: None,
            description: Some(
                "FLASH key register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "key",
                    description: Some(
                        "Non-volatile memoryconfiguration access unlock key.",
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
            name: "Oemkeyr1Prg",
            extends: None,
            description: Some(
                "FLASH OEM Key register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "oemkey",
                    description: Some(
                        "Least significants bytes of OEMKEY.",
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
            name: "Oemkeyr2Prg",
            extends: None,
            description: Some(
                "FLASH OEM Key register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "oemkey",
                    description: Some(
                        "Mid-least significants bytes of OEMKEY.",
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
            name: "Oemkeyr3Prg",
            extends: None,
            description: Some(
                "FLASH OEM Key register 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "oemkey",
                    description: Some(
                        "Mid-most significants bytes of OEMKEY.",
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
            name: "Oemkeyr4Prg",
            extends: None,
            description: Some(
                "FLASH OEM Key register 4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "oemkey",
                    description: Some(
                        "Most significants bytes of OEMKEY.",
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
            name: "Opsr",
            extends: None,
            description: Some(
                "FLASH operation status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addr_op",
                    description: Some(
                        "Interrupted operation address.",
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
                    name: "data_op",
                    description: Some(
                        "Flash data area operation interrupted.",
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
                    name: "bk_op",
                    description: Some(
                        "Interrupted operation bank.",
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
                    name: "sysf_op",
                    description: Some(
                        "Operation in system flash memory interrupted.",
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
                    name: "otp_op",
                    description: Some(
                        "OTP operation interrupted.",
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
                    name: "code_op",
                    description: Some(
                        "Flash memory operation code.",
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
            name: "Optcr",
            extends: None,
            description: Some(
                "FLASH option control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "optlock",
                    description: Some(
                        "OPTCR lock option configuration bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Optlock",
                    ),
                },
                Field {
                    name: "optstrt",
                    description: Some(
                        "Option-byte start change option configuration bit.",
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
                    name: "swap_bank",
                    description: Some(
                        "Bank swapping option configuration bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "OptcrSwapBank",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Optkeyr",
            extends: None,
            description: Some(
                "FLASH option key register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "optkey",
                    description: Some(
                        "FLASH option-byte control access unlock key.",
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
            name: "Optsr2Cur",
            extends: None,
            description: Some(
                "FLASH option status register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sram1_rst",
                    description: Some(
                        "SRAM1 erase upon system reset.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Optsr2CurSram1Rst",
                    ),
                },
                Field {
                    name: "sram2_rst",
                    description: Some(
                        "SRAM2 erase when system reset.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Optsr2CurSram2Rst",
                    ),
                },
                Field {
                    name: "sram2_ecc",
                    description: Some(
                        "SRAM2 ECC detection and correction disable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Optsr2CurSram2Ecc",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Optsr2Prg",
            extends: None,
            description: Some(
                "FLASH option status register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sram1_rst",
                    description: Some(
                        "SRAM1 erase upon system reset.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Optsr2PrgSram1Rst",
                    ),
                },
                Field {
                    name: "sram2_rst",
                    description: Some(
                        "SRAM2 erase when system reset.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Optsr2PrgSram2Rst",
                    ),
                },
                Field {
                    name: "sram2_ecc",
                    description: Some(
                        "SRAM2 ECC detection and correction disable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Optsr2PrgSram2Ecc",
                    ),
                },
            ],
        },
        FieldSet {
            name: "OptsrCur",
            extends: None,
            description: Some(
                "FLASH option status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iwdg_sw",
                    description: Some(
                        "IWDG control mode option status bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "OptsrCurIwdgSw",
                    ),
                },
                Field {
                    name: "wwdg_sw",
                    description: Some(
                        "WWDG control mode option status bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "OptsrCurWwdgSw",
                    ),
                },
                Field {
                    name: "nrst_stop",
                    description: Some(
                        "Core domain Stop entry reset option status bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "OptsrCurNrstStop",
                    ),
                },
                Field {
                    name: "nrst_stdby",
                    description: Some(
                        "Core domain Standby entry reset option status bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "OptsrCurNrstStdby",
                    ),
                },
                Field {
                    name: "rdp_level",
                    description: Some(
                        "RDP level code (based on Hamming 8,4). See Section7.5.8.",
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
                    name: "iwdg_stop",
                    description: Some(
                        "IWDG Stop mode freeze option status bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "OptsrCurIwdgStop",
                    ),
                },
                Field {
                    name: "iwdg_stdby",
                    description: Some(
                        "IWDG Standby mode freeze option status bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "OptsrCurIwdgStdby",
                    ),
                },
                Field {
                    name: "boot_sel",
                    description: Some(
                        "Boot 0 source selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "OptsrCurBootSel",
                    ),
                },
                Field {
                    name: "boot0",
                    description: Some(
                        "Boot 0 option bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "OptsrCurBoot0",
                    ),
                },
                Field {
                    name: "edata_en",
                    description: Some(
                        "Flash data area enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "OptsrCurEdataEn",
                    ),
                },
                Field {
                    name: "dual_bank",
                    description: Some(
                        "Dual bank selection option status bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "OptsrCurDualBank",
                    ),
                },
                Field {
                    name: "single_bank",
                    description: Some(
                        "Dual bank selection option status bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "OptsrCurSingleBank",
                    ),
                },
                Field {
                    name: "swap_bank",
                    description: Some(
                        "Bank swapping option status bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "OptsrCurSwapBank",
                    ),
                },
            ],
        },
        FieldSet {
            name: "OptsrPrg",
            extends: None,
            description: Some(
                "FLASH option status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iwdg_sw",
                    description: Some(
                        "IWDG control mode option configuration bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "OptsrPrgIwdgSw",
                    ),
                },
                Field {
                    name: "wwdg_sw",
                    description: Some(
                        "WWDG control mode option configuration bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "OptsrPrgWwdgSw",
                    ),
                },
                Field {
                    name: "nrst_stop",
                    description: Some(
                        "Core domain Stop entry reset option configuration bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "OptsrPrgNrstStop",
                    ),
                },
                Field {
                    name: "nrst_stdby",
                    description: Some(
                        "Core domain Standby entry reset option configuration bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "OptsrPrgNrstStdby",
                    ),
                },
                Field {
                    name: "rdp_level",
                    description: Some(
                        "RDP level code (based on Hamming 8,4). See Section7.5.8.",
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
                    name: "iwdg_stop",
                    description: Some(
                        "IWDG Stop mode freeze option configuration bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "OptsrPrgIwdgStop",
                    ),
                },
                Field {
                    name: "iwdg_stdby",
                    description: Some(
                        "IWDG Standby mode freeze option configuration bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "OptsrPrgIwdgStdby",
                    ),
                },
                Field {
                    name: "boot_sel",
                    description: Some(
                        "Boot 0 source configuration.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "OptsrPrgBootSel",
                    ),
                },
                Field {
                    name: "boot0",
                    description: Some(
                        "Boot 0 option bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "OptsrPrgBoot0",
                    ),
                },
                Field {
                    name: "edata_en",
                    description: Some(
                        "Flash data area enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "OptsrPrgEdataEn",
                    ),
                },
                Field {
                    name: "dual_bank",
                    description: Some(
                        "Dual bank option configuration bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "OptsrPrgDualBank",
                    ),
                },
                Field {
                    name: "single_bank",
                    description: Some(
                        "Dual bank option configuration bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "OptsrPrgSingleBank",
                    ),
                },
                Field {
                    name: "swap_bank",
                    description: Some(
                        "Bank swapping option configuration bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "OptsrPrgSwapBank",
                    ),
                },
            ],
        },
        FieldSet {
            name: "OtpblrCur",
            extends: None,
            description: Some(
                "FLASH OTP block lock.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lockbl",
                    description: Some(
                        "OTP block lock.",
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
            name: "OtpblrPrg",
            extends: None,
            description: Some(
                "FLASH OTP block lock.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lockbl",
                    description: Some(
                        "OTP block lock.",
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
            name: "Privcfgr",
            extends: None,
            description: Some(
                "FLASH privilege configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "priv_",
                    description: Some(
                        "privilege attribute.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Priv",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Sr",
            extends: None,
            description: Some(
                "FLASH status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bsy",
                    description: Some(
                        "busy flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Bsy",
                    ),
                },
                Field {
                    name: "wbne",
                    description: Some(
                        "write buffer not empty flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Wbne",
                    ),
                },
                Field {
                    name: "dbne",
                    description: Some(
                        "data buffer not empty flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Dbne",
                    ),
                },
                Field {
                    name: "oemlock",
                    description: Some(
                        "OEM lock.",
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
                    name: "bslock",
                    description: Some(
                        "BS lock.",
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
                    name: "eop",
                    description: Some(
                        "end of operation flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Eop",
                    ),
                },
                Field {
                    name: "wrperr",
                    description: Some(
                        "write protection error flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Wrperr",
                    ),
                },
                Field {
                    name: "pgserr",
                    description: Some(
                        "programming sequence error flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pgserr",
                    ),
                },
                Field {
                    name: "strberr",
                    description: Some(
                        "strobe error flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Strberr",
                    ),
                },
                Field {
                    name: "incerr",
                    description: Some(
                        "Inconsistency error flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Incerr",
                    ),
                },
                Field {
                    name: "optchangeerr",
                    description: Some(
                        "Option-byte change error flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Optchangeerr",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Wrp1rCur",
            extends: None,
            description: Some(
                "FLASH write page protection for bank1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wrpsg1",
                    description: Some(
                        "Bank1 page protection option status byte.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: Some(
                        "Wrp1rCurWrpsg1",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Wrp1rPrg",
            extends: None,
            description: Some(
                "FLASH write page protection for bank1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wrpsg1",
                    description: Some(
                        "Bank1 page protection option status byte.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: Some(
                        "Wrp1rPrgWrpsg1",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Wrp2rCur",
            extends: None,
            description: Some(
                "FLASH write page protection for bank2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wrpsg2",
                    description: Some(
                        "Bank2 page protection option status byte.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: Some(
                        "Wrp2rCurWrpsg2",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Wrp2rPrg",
            extends: None,
            description: Some(
                "FLASH write page protection for bank2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wrpsg2",
                    description: Some(
                        "Bank2 page protection option status byte.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: Some(
                        "Wrp2rPrgWrpsg2",
                    ),
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Ber",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "bank erase not requested.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "bank erase requested.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Bksel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "Bank1 is selected for bank erase (BER)/page erase (PER)/interrupt enable.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "Bank2 is selected for BER/PER.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "BootrCurBootLock",
            description: None,
            bit_size: 8,
            variants: &[
                EnumVariant {
                    name: "B0xB4",
                    description: Some(
                        "The BOOT0, BOOT_SEL, SWAP_BANK and BOOTADD are frozen.",
                    ),
                    value: 180,
                },
                EnumVariant {
                    name: "B0xC3",
                    description: Some(
                        "The BOOT0, BOOT_SEL, SWAP_BANK and BOOTADD can still be modified following their individual rules.",
                    ),
                    value: 195,
                },
            ],
        },
        Enum {
            name: "BootrPrgBootLock",
            description: None,
            bit_size: 8,
            variants: &[
                EnumVariant {
                    name: "B0xB4",
                    description: Some(
                        "The BOOT0, BOOT_SEL, SWAP_BANK and BOOTADD are frozen.",
                    ),
                    value: 180,
                },
                EnumVariant {
                    name: "B0xC3",
                    description: Some(
                        "The BOOT0, BOOT_SEL, SWAP_BANK and BOOTADD can still be modified following their individual rules.",
                    ),
                    value: 195,
                },
            ],
        },
        Enum {
            name: "Bsy",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "no programming, erase or option byte change operation being executed.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "programming, erase or option byte change operation being executed.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "CodeOp",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "No FLASH operation ongoing during previous reset.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "Single write operation interrupted.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B0x3",
                    description: Some(
                        "Page erase operation interrupted.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "B0x4",
                    description: Some(
                        "Bank erase operation interrupted.",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "B0x5",
                    description: Some(
                        "Mass erase operation interrupted.",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "B0x6",
                    description: Some(
                        "Option change operation interrupted.",
                    ),
                    value: 6,
                },
            ],
        },
        Enum {
            name: "DataAddrEcc",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "Double error on first 16bits data or first 32 bits data accessed on the FLASH line.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "Double error on second 16bits data accessed on the FLASH line.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B0x2",
                    description: Some(
                        "Double error on third 16bits data or second 32 bits data accessed on the FLASH line.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "B0x3",
                    description: Some(
                        "Double error on fourth 16bits data accessed on the FLASH line.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "B0x4",
                    description: Some(
                        "Double error on fifth 16bits data or third 32 bits data accessed on the FLASH line.",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "B0x5",
                    description: Some(
                        "Double error on sixth 16bits data.",
                    ),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Dbne",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "data buffer not used.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "data buffer used, wait.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ecccie",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "no interrupt generated when an ECC single correction error occurs.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "interrupt generated when an ECC single correction error occurs.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Edatasel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "Main flash page erase.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "Flash data area EDATA page erase.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Empty",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "Boot address in Main Flash memory area programmed.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "Boot address in Main Flash memory area empty.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Eop",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "no operation completed.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "a operation completed.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Eopie",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "no interrupt generated at the end of operation.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "interrupt enabled when at the end of operation.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Incerr",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "no inconsistency error occurs.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "a inconsistency error occurs.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Incerrie",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "no interrupt generated when a inconsistency error occurs.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "interrupt generated when a inconsistency error occurs.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Latency",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "zero wait state used to read a word from non-volatile memory.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "one wait state used to read a word from non-volatile memory.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B0x2",
                    description: Some(
                        "two wait states used to read a word from non-volatile memory.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "B0x7",
                    description: Some(
                        "seven wait states used to read a word from non-volatile memory.",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "B0xF",
                    description: Some(
                        "15 wait states used to read from non-volatile memory.",
                    ),
                    value: 15,
                },
            ],
        },
        Enum {
            name: "Lock",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "CR register unlocked.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "CR register locked.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Mer",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "mass erase not requested.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "mass erase requested.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Optchangeerr",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "no option-byte change errors occurred.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "one or more errors occurred during an option-byte change operation.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Optchangeerrie",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "no interrupt is generated when an error occurs during an option-byte change.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "an interrupt is generated when and error occurs during an option-byte change.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "OptcrSwapBank",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "Bank1 and bank2 not swapped.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "Bank1 and bank2 swapped.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Optlock",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "OPTCR register unlocked.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "OPTCR register locked.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Optsr2CurSram1Rst",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "SRAM1 erased when a system reset occurs.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "SRAM1 not erased when a system reset occurs.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Optsr2CurSram2Ecc",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "SRAM2 ECC check enabled.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "SRAM2 ECC check disabled.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Optsr2CurSram2Rst",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "SRAM2 erased when a system reset occurs.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "SRAM2 not erased when a system reset occurs.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Optsr2PrgSram1Rst",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "SRAM1 erased when a system reset occurs.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "SRAM1 not erased when a system reset occurs.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Optsr2PrgSram2Ecc",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "SRAM2 ECC check enabled.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "SRAM2 ECC check disabled.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Optsr2PrgSram2Rst",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "SRAM2 erased when a system reset occurs.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "SRAM2 not erased when a system reset occurs.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "OptsrCurBoot0",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "BOOT0 = 0.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "BOOT0 = 1.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "OptsrCurBootSel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "BOOT0 signal is defined by the BOOT0 option bit.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "BOOT0 signal is defined by BOOT0 pin value (legacy mode).",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "OptsrCurDualBank",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "256Kbytes of user flash located in one bank.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "256Kbytes of user flash split with 128Kbytes in Bank 1 and 128Kbytes in Bank 2.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "OptsrCurEdataEn",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "No flash data area (EDATA pages are 128 bits writable).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "Flash data area is enabled (EDATA pages are 16/32 bits writable).",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "OptsrCurIwdgStdby",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "Independent watchdog frozen in Standby mode.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "Independent watchdog keep running in Standby mode.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "OptsrCurIwdgStop",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "Independent watchdog frozen in system Stop mode.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "Independent watchdog keep running in system Stop mode.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "OptsrCurIwdgSw",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "IWDG watchdog is controlled by hardware.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "IWDG watchdog is controlled by software.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "OptsrCurNrstStdby",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "a reset is generated when entering Standby mode on core domain.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "no reset generated when entering Standby mode on core domain.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "OptsrCurNrstStop",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "a reset is generated when entering Stop mode on core domain.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "no reset generated when entering Stop mode on core domain.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "OptsrCurSingleBank",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "128 Kbytes of user flash split with 64 Kbytes in Bank 1 and 64 Kbytes in Bank 2.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "128 Kbytes of user flash located in one bank.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "OptsrCurSwapBank",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "Bank1 and bank2 not swapped.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "Bank1 and bank2 swapped.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "OptsrCurWwdgSw",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "WWDG watchdog is controlled by hardware.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "WWDG watchdog is controlled by software.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "OptsrPrgBoot0",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "BOOT0 = 0.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "BOOT0 = 1.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "OptsrPrgBootSel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "BOOT0 signal is defined by the BOOT0 option bit.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "BOOT0 signal is defined by BOOT0 pin value (legacy mode).",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "OptsrPrgDualBank",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "256Kbytes of user flash located in one bank.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "256Kbytes of user flash split with 128Kbytes in Bank 1 and 128Kbytes in Bank 2.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "OptsrPrgEdataEn",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "No flash data area.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "Flash data area is enabled.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "OptsrPrgIwdgStdby",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "Independent watchdog frozen in Standby mode.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "Independent watchdog keep running in Standby mode.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "OptsrPrgIwdgStop",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "Independent watchdog frozen in system Stop mode.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "Independent watchdog keep running in system Stop mode.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "OptsrPrgIwdgSw",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "IWDG watchdog is controlled by hardware.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "IWDG watchdog is controlled by software.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "OptsrPrgNrstStdby",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "a reset is generated when entering Standby mode on core domain.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "no reset generated when entering Standby mode on core domain.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "OptsrPrgNrstStop",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "a reset is generated when entering Stop mode on core domain.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "no reset generated when entering Stop mode on core domain.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "OptsrPrgSingleBank",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "128 Kbytes of user flash split with 64 Kbytes in Bank 1 and 64 Kbytes in Bank 2.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "128 Kbytes of user flash located in one bank.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "OptsrPrgSwapBank",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "Bank1 and bank2 not swapped.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "Bank1 and bank2 swapped.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "OptsrPrgWwdgSw",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "WWDG watchdog is controlled by hardware.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "WWDG watchdog is controlled by software.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Per",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "page erase not requested.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "page erase requested.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pg",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "programming disabled.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "programming enabled.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pgserr",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "no sequence error occurred.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "a sequence error occurred.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pgserrie",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "no interrupt generated when a sequence error occurs.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "interrupt generated when sequence error occurs.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pnb",
            description: None,
            bit_size: 6,
            variants: &[
                EnumVariant {
                    name: "B0x00",
                    description: Some(
                        "Page 0 selected.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x01",
                    description: Some(
                        "Page 1 selected.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Prften",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "prefetch disabled.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "prefetch enabled when latency is at least one wait-state.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Priv",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "Access to registers is always granted.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "Access to registers is denied in case of unprivileged access.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ser",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "sector erase not requested.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "sector erase requested.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Snb",
            description: None,
            bit_size: 6,
            variants: &[
                EnumVariant {
                    name: "B0x00",
                    description: Some(
                        "Sector 0 selected.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x01",
                    description: Some(
                        "Sector 1 selected.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B0x2f",
                    description: Some(
                        "Sector 41 selected.",
                    ),
                    value: 47,
                },
            ],
        },
        Enum {
            name: "Strberr",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "no strobe error occurred.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "a strobe error occurred.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Strberrie",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "no interrupt generated when a strobe error occurs.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "interrupt generated when strobe error occurs.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Wbne",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "write buffer empty or full.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "write buffer waiting data to complete.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Wrp1rCurWrpsg1",
            description: None,
            bit_size: 32,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "write protected;.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "not write protected.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Wrp1rPrgWrpsg1",
            description: None,
            bit_size: 32,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "write protected;.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "not write protected.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Wrp2rCurWrpsg2",
            description: None,
            bit_size: 32,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "write protected.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "not write protected.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Wrp2rPrgWrpsg2",
            description: None,
            bit_size: 32,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "write protected;.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "not write protected.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Wrperr",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "no write protection error occurred.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "a write protection error occurred.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Wrperrie",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "no interrupt generated when a protection error occurs.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "interrupt generated when a protection error occurs.",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
