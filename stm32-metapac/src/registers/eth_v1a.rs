
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "EthernetDma",
            extends: None,
            description: Some(
                "Ethernet: DMA controller operation",
            ),
            items: &[
                BlockItem {
                    name: "dmabmr",
                    description: Some(
                        "Ethernet DMA bus mode register",
                    ),
                    array: None,
                    byte_offset: 0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmabmr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmatpdr",
                    description: Some(
                        "Ethernet DMA transmit poll demand register",
                    ),
                    array: None,
                    byte_offset: 4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmatpdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmarpdr",
                    description: Some(
                        "EHERNET DMA receive poll demand register",
                    ),
                    array: None,
                    byte_offset: 8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmarpdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmardlar",
                    description: Some(
                        "Ethernet DMA receive descriptor list address register",
                    ),
                    array: None,
                    byte_offset: 12,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmardlar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmatdlar",
                    description: Some(
                        "Ethernet DMA transmit descriptor list address register",
                    ),
                    array: None,
                    byte_offset: 16,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmatdlar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmasr",
                    description: Some(
                        "Ethernet DMA status register",
                    ),
                    array: None,
                    byte_offset: 20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmasr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmaomr",
                    description: Some(
                        "Ethernet DMA operation mode register",
                    ),
                    array: None,
                    byte_offset: 24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmaomr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmaier",
                    description: Some(
                        "Ethernet DMA interrupt enable register",
                    ),
                    array: None,
                    byte_offset: 28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmaier",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmamfbocr",
                    description: Some(
                        "Ethernet DMA missed frame and buffer overflow counter register",
                    ),
                    array: None,
                    byte_offset: 32,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmamfbocr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmachtdr",
                    description: Some(
                        "Ethernet DMA current host transmit descriptor register",
                    ),
                    array: None,
                    byte_offset: 72,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmachtdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmachrdr",
                    description: Some(
                        "Ethernet DMA current host receive descriptor register",
                    ),
                    array: None,
                    byte_offset: 76,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmachrdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmachtbar",
                    description: Some(
                        "Ethernet DMA current host transmit buffer address register",
                    ),
                    array: None,
                    byte_offset: 80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmachtbar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmachrbar",
                    description: Some(
                        "Ethernet DMA current host receive buffer address register",
                    ),
                    array: None,
                    byte_offset: 84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmachrbar",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "EthernetMac",
            extends: None,
            description: Some(
                "Ethernet: media access control (MAC)",
            ),
            items: &[
                BlockItem {
                    name: "maccr",
                    description: Some(
                        "Ethernet MAC configuration register",
                    ),
                    array: None,
                    byte_offset: 0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Maccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macffr",
                    description: Some(
                        "Ethernet MAC frame filter register",
                    ),
                    array: None,
                    byte_offset: 4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macffr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "machthr",
                    description: Some(
                        "Ethernet MAC hash table high register",
                    ),
                    array: None,
                    byte_offset: 8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Machthr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "machtlr",
                    description: Some(
                        "Ethernet MAC hash table low register",
                    ),
                    array: None,
                    byte_offset: 12,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Machtlr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macmiiar",
                    description: Some(
                        "Ethernet MAC MII address register",
                    ),
                    array: None,
                    byte_offset: 16,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macmiiar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macmiidr",
                    description: Some(
                        "Ethernet MAC MII data register",
                    ),
                    array: None,
                    byte_offset: 20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macmiidr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macfcr",
                    description: Some(
                        "Ethernet MAC flow control register",
                    ),
                    array: None,
                    byte_offset: 24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macfcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macvlantr",
                    description: Some(
                        "Ethernet MAC VLAN tag register",
                    ),
                    array: None,
                    byte_offset: 28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macvlantr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macrwuffr",
                    description: Some(
                        "Ethernet MAC remote wakeup frame filter register",
                    ),
                    array: None,
                    byte_offset: 40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "macpmtcsr",
                    description: Some(
                        "Ethernet MAC PMT control and status register",
                    ),
                    array: None,
                    byte_offset: 44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macpmtcsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macdbgr",
                    description: Some(
                        "Ethernet MAC debug register",
                    ),
                    array: None,
                    byte_offset: 52,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Macdbgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macsr",
                    description: Some(
                        "Ethernet MAC interrupt status register",
                    ),
                    array: None,
                    byte_offset: 56,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macimr",
                    description: Some(
                        "Ethernet MAC interrupt mask register",
                    ),
                    array: None,
                    byte_offset: 60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macimr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "maca0hr",
                    description: Some(
                        "Ethernet MAC address 0 high register",
                    ),
                    array: None,
                    byte_offset: 64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Maca0hr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "maca0lr",
                    description: Some(
                        "Ethernet MAC address 0 low register",
                    ),
                    array: None,
                    byte_offset: 68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Maca0lr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "maca1hr",
                    description: Some(
                        "Ethernet MAC address 1 high register",
                    ),
                    array: None,
                    byte_offset: 72,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Maca1hr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "maca1lr",
                    description: Some(
                        "Ethernet MAC address1 low register",
                    ),
                    array: None,
                    byte_offset: 76,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Maca1lr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "maca2hr",
                    description: Some(
                        "Ethernet MAC address 2 high register",
                    ),
                    array: None,
                    byte_offset: 80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Maca2hr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "maca2lr",
                    description: Some(
                        "Ethernet MAC address 2 low register",
                    ),
                    array: None,
                    byte_offset: 84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Maca2lr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "maca3hr",
                    description: Some(
                        "Ethernet MAC address 3 high register",
                    ),
                    array: None,
                    byte_offset: 88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Maca3hr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "maca3lr",
                    description: Some(
                        "Ethernet MAC address 3 low register",
                    ),
                    array: None,
                    byte_offset: 92,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Maca3lr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmccr",
                    description: Some(
                        "Ethernet MMC control register",
                    ),
                    array: None,
                    byte_offset: 256,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mmccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmcrir",
                    description: Some(
                        "Ethernet MMC receive interrupt register",
                    ),
                    array: None,
                    byte_offset: 260,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mmcrir",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmctir",
                    description: Some(
                        "Ethernet MMC transmit interrupt register",
                    ),
                    array: None,
                    byte_offset: 264,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Mmctir",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmcrimr",
                    description: Some(
                        "Ethernet MMC receive interrupt mask register",
                    ),
                    array: None,
                    byte_offset: 268,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mmcrimr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmctimr",
                    description: Some(
                        "Ethernet MMC transmit interrupt mask register",
                    ),
                    array: None,
                    byte_offset: 272,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mmctimr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmctgfsccr",
                    description: Some(
                        "Ethernet MMC transmitted good frames after a single collision counter",
                    ),
                    array: None,
                    byte_offset: 332,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Mmctgfsccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmctgfmsccr",
                    description: Some(
                        "Ethernet MMC transmitted good frames after more than a single collision",
                    ),
                    array: None,
                    byte_offset: 336,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Mmctgfmsccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmctgfcr",
                    description: Some(
                        "Ethernet MMC transmitted good frames counter register",
                    ),
                    array: None,
                    byte_offset: 360,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Mmctgfcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmcrfcecr",
                    description: Some(
                        "Ethernet MMC received frames with CRC error counter register",
                    ),
                    array: None,
                    byte_offset: 404,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Mmcrfcecr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmcrfaecr",
                    description: Some(
                        "Ethernet MMC received frames with alignment error counter register",
                    ),
                    array: None,
                    byte_offset: 408,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Mmcrfaecr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mmcrgufcr",
                    description: Some(
                        "MMC received good unicast frames counter register",
                    ),
                    array: None,
                    byte_offset: 452,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Mmcrgufcr",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Eth",
            extends: None,
            description: Some(
                "Ethernet Peripheral",
            ),
            items: &[
                BlockItem {
                    name: "ethernet_mac",
                    description: Some(
                        "Ethernet: media access control (MAC)",
                    ),
                    array: None,
                    byte_offset: 0,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "EthernetMac",
                        },
                    ),
                },
                BlockItem {
                    name: "ethernet_ptp",
                    description: Some(
                        "Ethernet: Precision Time Protocol (PTP)",
                    ),
                    array: None,
                    byte_offset: 1792,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "EthernetPtp",
                        },
                    ),
                },
                BlockItem {
                    name: "ethernet_dma",
                    description: Some(
                        "Ethernet: DMA mode register (DMA)",
                    ),
                    array: None,
                    byte_offset: 4096,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "EthernetDma",
                        },
                    ),
                },
            ],
        },
        Block {
            name: "EthernetPtp",
            extends: None,
            description: Some(
                "Ethernet: Precision time protocol",
            ),
            items: &[
                BlockItem {
                    name: "ptptscr",
                    description: Some(
                        "Ethernet PTP time stamp control register",
                    ),
                    array: None,
                    byte_offset: 0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ptptscr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptpssir",
                    description: Some(
                        "Ethernet PTP subsecond increment register",
                    ),
                    array: None,
                    byte_offset: 4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ptpssir",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptptshr",
                    description: Some(
                        "Ethernet PTP time stamp high register",
                    ),
                    array: None,
                    byte_offset: 8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Ptptshr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptptslr",
                    description: Some(
                        "Ethernet PTP time stamp low register",
                    ),
                    array: None,
                    byte_offset: 12,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Ptptslr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptptshur",
                    description: Some(
                        "Ethernet PTP time stamp high update register",
                    ),
                    array: None,
                    byte_offset: 16,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ptptshur",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptptslur",
                    description: Some(
                        "Ethernet PTP time stamp low update register",
                    ),
                    array: None,
                    byte_offset: 20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ptptslur",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptptsar",
                    description: Some(
                        "Ethernet PTP time stamp addend register",
                    ),
                    array: None,
                    byte_offset: 24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ptptsar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptptthr",
                    description: Some(
                        "Ethernet PTP target time high register",
                    ),
                    array: None,
                    byte_offset: 28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ptptthr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptpttlr",
                    description: Some(
                        "Ethernet PTP target time low register",
                    ),
                    array: None,
                    byte_offset: 32,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ptpttlr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptptssr",
                    description: Some(
                        "Ethernet PTP time stamp status register",
                    ),
                    array: None,
                    byte_offset: 40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Ptptssr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptpppscr",
                    description: Some(
                        "Ethernet PTP PPS control register",
                    ),
                    array: None,
                    byte_offset: 44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Ptpppscr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Macsr",
            extends: None,
            description: Some(
                "Ethernet MAC interrupt status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pmts",
                    description: Some(
                        "PMT status",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mmcs",
                    description: Some(
                        "MMC status",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mmcrs",
                    description: Some(
                        "MMC receive status",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mmcts",
                    description: Some(
                        "MMC transmit status",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tsts",
                    description: Some(
                        "Time stamp trigger status",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dmaomr",
            extends: None,
            description: Some(
                "Ethernet DMA operation mode register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sr",
                    description: Some(
                        "Start/stop receive",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "DmaomrSr",
                    ),
                },
                Field {
                    name: "osf",
                    description: Some(
                        "Operate on second frame",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rtc",
                    description: Some(
                        "Receive threshold control",
                    ),
                    bit_offset: 3,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Rtc",
                    ),
                },
                Field {
                    name: "fugf",
                    description: Some(
                        "Forward undersized good frames",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Fugf",
                    ),
                },
                Field {
                    name: "fef",
                    description: Some(
                        "Forward error frames",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Fef",
                    ),
                },
                Field {
                    name: "st",
                    description: Some(
                        "Start/stop transmission",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "St",
                    ),
                },
                Field {
                    name: "ttc",
                    description: Some(
                        "Transmit threshold control",
                    ),
                    bit_offset: 14,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Ttc",
                    ),
                },
                Field {
                    name: "ftf",
                    description: Some(
                        "Flush transmit FIFO",
                    ),
                    bit_offset: 20,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ftf",
                    ),
                },
                Field {
                    name: "tsf",
                    description: Some(
                        "Transmit store and forward",
                    ),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Tsf",
                    ),
                },
                Field {
                    name: "dfrf",
                    description: Some(
                        "Disable flushing of received frames",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rsf",
                    description: Some(
                        "Receive store and forward",
                    ),
                    bit_offset: 25,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Rsf",
                    ),
                },
                Field {
                    name: "dtcefd",
                    description: Some(
                        "Dropping of TCP/IP checksum error frames disable",
                    ),
                    bit_offset: 26,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Dtcefd",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Ptptshur",
            extends: None,
            description: Some(
                "Ethernet PTP time stamp high update register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tsus",
                    description: Some(
                        "TSUS",
                    ),
                    bit_offset: 0,
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Mmctimr",
            extends: None,
            description: Some(
                "Ethernet MMC transmit interrupt mask register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tgfscm",
                    description: Some(
                        "Transmitted good frames single collision mask",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Tgfscm",
                    ),
                },
                Field {
                    name: "tgfmscm",
                    description: Some(
                        "Transmitted good frames more than single collision mask",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Tgfmscm",
                    ),
                },
                Field {
                    name: "tgfm",
                    description: Some(
                        "Transmitted good frames mask",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Tgfm",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Ptpssir",
            extends: None,
            description: Some(
                "Ethernet PTP subsecond increment register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "stssi",
                    description: Some(
                        "STSSI",
                    ),
                    bit_offset: 0,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Macvlantr",
            extends: None,
            description: Some(
                "Ethernet MAC VLAN tag register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vlanti",
                    description: Some(
                        "VLAN tag identifier (for receive frames)",
                    ),
                    bit_offset: 0,
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vlantc",
                    description: Some(
                        "12-bit VLAN tag comparison",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Vlantc",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Ptptshr",
            extends: None,
            description: Some(
                "Ethernet PTP time stamp high register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sts",
                    description: Some(
                        "STS",
                    ),
                    bit_offset: 0,
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dmaier",
            extends: None,
            description: Some(
                "Ethernet DMA interrupt enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tie",
                    description: Some(
                        "Transmit interrupt enable",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tpsie",
                    description: Some(
                        "Transmit process stopped interrupt enable",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tbuie",
                    description: Some(
                        "Transmit buffer unavailable interrupt enable",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tjtie",
                    description: Some(
                        "Transmit jabber timeout interrupt enable",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "roie",
                    description: Some(
                        "Receive overflow interrupt enable",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tuie",
                    description: Some(
                        "Transmit underflow interrupt enable",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rie",
                    description: Some(
                        "Receive interrupt enable",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rbuie",
                    description: Some(
                        "Receive buffer unavailable interrupt enable",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rpsie",
                    description: Some(
                        "Receive process stopped interrupt enable",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rwtie",
                    description: Some(
                        "Receive watchdog timeout interrupt enable",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "etie",
                    description: Some(
                        "Early transmit interrupt enable",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fbeie",
                    description: Some(
                        "Fatal bus error interrupt enable",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "erie",
                    description: Some(
                        "Early receive interrupt enable",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aise",
                    description: Some(
                        "Abnormal interrupt summary enable",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nise",
                    description: Some(
                        "Normal interrupt summary enable",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Macmiiar",
            extends: None,
            description: Some(
                "Ethernet MAC MII address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mb",
                    description: Some(
                        "MII busy",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "MbProgress",
                    ),
                },
                Field {
                    name: "mw",
                    description: Some(
                        "MII write",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Mw",
                    ),
                },
                Field {
                    name: "cr",
                    description: Some(
                        "Clock range",
                    ),
                    bit_offset: 2,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Cr",
                    ),
                },
                Field {
                    name: "mr",
                    description: Some(
                        "MII register - select the desired MII register in the PHY device",
                    ),
                    bit_offset: 6,
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pa",
                    description: Some(
                        "PHY address - select which of possible 32 PHYs is being accessed",
                    ),
                    bit_offset: 11,
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Maca1lr",
            extends: None,
            description: Some(
                "Ethernet MAC address1 low register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "maca1l",
                    description: Some(
                        "MACA1LR",
                    ),
                    bit_offset: 0,
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Mmcrfcecr",
            extends: None,
            description: Some(
                "Ethernet MMC received frames with CRC error counter register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rfcfc",
                    description: Some(
                        "RFCFC",
                    ),
                    bit_offset: 0,
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Maca1hr",
            extends: None,
            description: Some(
                "Ethernet MAC address 1 high register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "maca1h",
                    description: Some(
                        "MACA1H",
                    ),
                    bit_offset: 0,
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mbc",
                    description: Some(
                        "MBC",
                    ),
                    bit_offset: 24,
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sa",
                    description: Some(
                        "SA",
                    ),
                    bit_offset: 30,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "MacahrSa",
                    ),
                },
                Field {
                    name: "ae",
                    description: Some(
                        "AE",
                    ),
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "MacahrAe",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Maca3hr",
            extends: None,
            description: Some(
                "Ethernet MAC address 3 high register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "maca3h",
                    description: Some(
                        "MACA3H",
                    ),
                    bit_offset: 0,
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mbc",
                    description: Some(
                        "MBC",
                    ),
                    bit_offset: 24,
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sa",
                    description: Some(
                        "SA",
                    ),
                    bit_offset: 30,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "MacahrSa",
                    ),
                },
                Field {
                    name: "ae",
                    description: Some(
                        "AE",
                    ),
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "MacahrAe",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Macimr",
            extends: None,
            description: Some(
                "Ethernet MAC interrupt mask register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pmtim",
                    description: Some(
                        "PMT interrupt mask",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pmtim",
                    ),
                },
                Field {
                    name: "tstim",
                    description: Some(
                        "Time stamp trigger interrupt mask",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Tstim",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Mmctgfcr",
            extends: None,
            description: Some(
                "Ethernet MMC transmitted good frames counter register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tgfc",
                    description: Some(
                        "HTL",
                    ),
                    bit_offset: 0,
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dmachrbar",
            extends: None,
            description: Some(
                "Ethernet DMA current host receive buffer address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hrbap",
                    description: Some(
                        "Host receive buffer address pointer",
                    ),
                    bit_offset: 0,
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dmabmr",
            extends: None,
            description: Some(
                "Ethernet DMA bus mode register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sr",
                    description: Some(
                        "Software reset",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "da",
                    description: Some(
                        "DMA arbitration",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Da",
                    ),
                },
                Field {
                    name: "dsl",
                    description: Some(
                        "Descriptor skip length",
                    ),
                    bit_offset: 2,
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pbl",
                    description: Some(
                        "Programmable burst length",
                    ),
                    bit_offset: 8,
                    bit_size: 6,
                    array: None,
                    enumm: Some(
                        "Pbl",
                    ),
                },
                Field {
                    name: "pm",
                    description: Some(
                        "Rx-Tx priority ratio",
                    ),
                    bit_offset: 14,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "PriorityRxOverTx",
                    ),
                },
                Field {
                    name: "fb",
                    description: Some(
                        "Fixed burst",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Fb",
                    ),
                },
                Field {
                    name: "rdp",
                    description: Some(
                        "Rx DMA PBL",
                    ),
                    bit_offset: 17,
                    bit_size: 6,
                    array: None,
                    enumm: Some(
                        "Rdp",
                    ),
                },
                Field {
                    name: "usp",
                    description: Some(
                        "Use separate PBL",
                    ),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Usp",
                    ),
                },
                Field {
                    name: "fpm",
                    description: Some(
                        "4xPBL mode",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Fpm",
                    ),
                },
                Field {
                    name: "aab",
                    description: Some(
                        "Address-aligned beats",
                    ),
                    bit_offset: 25,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Aab",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Mmcrfaecr",
            extends: None,
            description: Some(
                "Ethernet MMC received frames with alignment error counter register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rfaec",
                    description: Some(
                        "RFAEC",
                    ),
                    bit_offset: 0,
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Mmctir",
            extends: None,
            description: Some(
                "Ethernet MMC transmit interrupt register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tgfscs",
                    description: Some(
                        "Transmitted good frames single collision status",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tgfmscs",
                    description: Some(
                        "Transmitted good frames more than single collision status",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tgfs",
                    description: Some(
                        "Transmitted good frames status",
                    ),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Mmccr",
            extends: None,
            description: Some(
                "Ethernet MMC control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cr",
                    description: Some(
                        "Counter reset",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "CounterReset",
                    ),
                },
                Field {
                    name: "csr",
                    description: Some(
                        "Counter stop rollover",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Csr",
                    ),
                },
                Field {
                    name: "ror",
                    description: Some(
                        "Reset on read",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ror",
                    ),
                },
                Field {
                    name: "mcf",
                    description: Some(
                        "MMC counter freeze",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Mcf",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Mmctgfsccr",
            extends: None,
            description: Some(
                "Ethernet MMC transmitted good frames after a single collision counter",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tgfscc",
                    description: Some(
                        "Transmitted good frames single collision counter",
                    ),
                    bit_offset: 0,
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Mmctgfmsccr",
            extends: None,
            description: Some(
                "Ethernet MMC transmitted good frames after more than a single collision",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tgfmscc",
                    description: Some(
                        "TGFMSCC",
                    ),
                    bit_offset: 0,
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ptptscr",
            extends: None,
            description: Some(
                "Ethernet PTP time stamp control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tse",
                    description: Some(
                        "TSE",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tsfcu",
                    description: Some(
                        "TSFCU",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tssti",
                    description: Some(
                        "TSSTI",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tsstu",
                    description: Some(
                        "TSSTU",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tsite",
                    description: Some(
                        "TSITE",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ttsaru",
                    description: Some(
                        "TTSARU",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tssarfe",
                    description: Some(
                        "TSSARFE",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tsssr",
                    description: Some(
                        "TSSSR",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tsptppsv2e",
                    description: Some(
                        "TSPTPPSV2E",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tssptpoefe",
                    description: Some(
                        "TSSPTPOEFE",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tssipv6fe",
                    description: Some(
                        "TSSIPV6FE",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tssipv4fe",
                    description: Some(
                        "TSSIPV4FE",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tsseme",
                    description: Some(
                        "TSSEME",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tssmrme",
                    description: Some(
                        "TSSMRME",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscnt",
                    description: Some(
                        "TSCNT",
                    ),
                    bit_offset: 16,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tspffmae",
                    description: Some(
                        "TSPFFMAE",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Mmcrir",
            extends: None,
            description: Some(
                "Ethernet MMC receive interrupt register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rfces",
                    description: Some(
                        "Received frames CRC error status",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rfaes",
                    description: Some(
                        "Received frames alignment error status",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rgufs",
                    description: Some(
                        "Received good Unicast frames status",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Macfcr",
            extends: None,
            description: Some(
                "Ethernet MAC flow control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fcb",
                    description: Some(
                        "Flow control busy/back pressure activate",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Fcb",
                    ),
                },
                Field {
                    name: "tfce",
                    description: Some(
                        "Transmit flow control enable",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Tfce",
                    ),
                },
                Field {
                    name: "rfce",
                    description: Some(
                        "Receive flow control enable",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Rfce",
                    ),
                },
                Field {
                    name: "upfd",
                    description: Some(
                        "Unicast pause frame detect",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Upfd",
                    ),
                },
                Field {
                    name: "plt",
                    description: Some(
                        "Pause low threshold",
                    ),
                    bit_offset: 4,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Plt",
                    ),
                },
                Field {
                    name: "zqpd",
                    description: Some(
                        "Zero-quanta pause disable",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Zqpd",
                    ),
                },
                Field {
                    name: "pt",
                    description: Some(
                        "Pause time",
                    ),
                    bit_offset: 16,
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Macpmtcsr",
            extends: None,
            description: Some(
                "Ethernet MAC PMT control and status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pd",
                    description: Some(
                        "Power down",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pd",
                    ),
                },
                Field {
                    name: "mpe",
                    description: Some(
                        "Magic packet enable",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Mpe",
                    ),
                },
                Field {
                    name: "wfe",
                    description: Some(
                        "Wakeup frame enable",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Wfe",
                    ),
                },
                Field {
                    name: "mpr",
                    description: Some(
                        "Magic packet received",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wfr",
                    description: Some(
                        "Wakeup frame received",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gu",
                    description: Some(
                        "Global unicast",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Gu",
                    ),
                },
                Field {
                    name: "wffrpr",
                    description: Some(
                        "Wakeup frame filter register pointer reset",
                    ),
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Wffrpr",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Dmachrdr",
            extends: None,
            description: Some(
                "Ethernet DMA current host receive descriptor register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hrdap",
                    description: Some(
                        "Host receive descriptor address pointer",
                    ),
                    bit_offset: 0,
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Macffr",
            extends: None,
            description: Some(
                "Ethernet MAC frame filter register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pm",
                    description: Some(
                        "Promiscuous mode",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pm",
                    ),
                },
                Field {
                    name: "hu",
                    description: Some(
                        "Hash unicast",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Hu",
                    ),
                },
                Field {
                    name: "hm",
                    description: Some(
                        "Hash multicast",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Hm",
                    ),
                },
                Field {
                    name: "daif",
                    description: Some(
                        "Destination address unique filtering",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Daif",
                    ),
                },
                Field {
                    name: "pam",
                    description: Some(
                        "Pass all multicast",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pam",
                    ),
                },
                Field {
                    name: "bfd",
                    description: Some(
                        "Broadcast frames disable",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Bfd",
                    ),
                },
                Field {
                    name: "pcf",
                    description: Some(
                        "Pass control frames",
                    ),
                    bit_offset: 6,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Pcf",
                    ),
                },
                Field {
                    name: "saif",
                    description: Some(
                        "Source address inverse filtering",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Saif",
                    ),
                },
                Field {
                    name: "saf",
                    description: Some(
                        "Source address filter",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Saf",
                    ),
                },
                Field {
                    name: "hpf",
                    description: Some(
                        "Hash or perfect filter",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Hpf",
                    ),
                },
                Field {
                    name: "ra",
                    description: Some(
                        "Receive all",
                    ),
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ra",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Ptptslr",
            extends: None,
            description: Some(
                "Ethernet PTP time stamp low register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "stss",
                    description: Some(
                        "STSS",
                    ),
                    bit_offset: 0,
                    bit_size: 31,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "stpns",
                    description: Some(
                        "STPNS",
                    ),
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Machtlr",
            extends: None,
            description: Some(
                "Ethernet MAC hash table low register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "htl",
                    description: Some(
                        "Lower 32 bits of hash table",
                    ),
                    bit_offset: 0,
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Mmcrgufcr",
            extends: None,
            description: Some(
                "MMC received good unicast frames counter register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rgufc",
                    description: Some(
                        "RGUFC",
                    ),
                    bit_offset: 0,
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Maca0lr",
            extends: None,
            description: Some(
                "Ethernet MAC address 0 low register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "maca0l",
                    description: Some(
                        "0",
                    ),
                    bit_offset: 0,
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dmardlar",
            extends: None,
            description: Some(
                "Ethernet DMA receive descriptor list address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "srl",
                    description: Some(
                        "Start of receive list",
                    ),
                    bit_offset: 0,
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Maccr",
            extends: None,
            description: Some(
                "Ethernet MAC configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "re",
                    description: Some(
                        "Receiver enable",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "te",
                    description: Some(
                        "Transmitter enable",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dc",
                    description: Some(
                        "Deferral check",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Dc",
                    ),
                },
                Field {
                    name: "bl",
                    description: Some(
                        "Back-off limit",
                    ),
                    bit_offset: 5,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Bl",
                    ),
                },
                Field {
                    name: "apcs",
                    description: Some(
                        "Automatic pad/CRC stripping",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Apcs",
                    ),
                },
                Field {
                    name: "rd",
                    description: Some(
                        "Retry disable",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Rd",
                    ),
                },
                Field {
                    name: "ipco",
                    description: Some(
                        "IPv4 checksum offload",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ipco",
                    ),
                },
                Field {
                    name: "dm",
                    description: Some(
                        "Duplex mode",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Dm",
                    ),
                },
                Field {
                    name: "lm",
                    description: Some(
                        "Loopback mode",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Lm",
                    ),
                },
                Field {
                    name: "rod",
                    description: Some(
                        "Receive own disable",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Rod",
                    ),
                },
                Field {
                    name: "fes",
                    description: Some(
                        "Fast Ethernet speed",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Fes",
                    ),
                },
                Field {
                    name: "csd",
                    description: Some(
                        "Carrier sense disable",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Csd",
                    ),
                },
                Field {
                    name: "ifg",
                    description: Some(
                        "Interframe gap",
                    ),
                    bit_offset: 17,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Ifg",
                    ),
                },
                Field {
                    name: "jd",
                    description: Some(
                        "Jabber disable",
                    ),
                    bit_offset: 22,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Jd",
                    ),
                },
                Field {
                    name: "wd",
                    description: Some(
                        "Watchdog disable",
                    ),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Wd",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Dmarpdr",
            extends: None,
            description: Some(
                "EHERNET DMA receive poll demand register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rpd",
                    description: Some(
                        "Receive poll demand",
                    ),
                    bit_offset: 0,
                    bit_size: 32,
                    array: None,
                    enumm: Some(
                        "Rpd",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Dmachtdr",
            extends: None,
            description: Some(
                "Ethernet DMA current host transmit descriptor register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "htdap",
                    description: Some(
                        "Host transmit descriptor address pointer",
                    ),
                    bit_offset: 0,
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dmatpdr",
            extends: None,
            description: Some(
                "Ethernet DMA transmit poll demand register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tpd",
                    description: Some(
                        "Transmit poll demand",
                    ),
                    bit_offset: 0,
                    bit_size: 32,
                    array: None,
                    enumm: Some(
                        "Tpd",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Machthr",
            extends: None,
            description: Some(
                "Ethernet MAC hash table high register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hth",
                    description: Some(
                        "Upper 32 bits of hash table",
                    ),
                    bit_offset: 0,
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Maca0hr",
            extends: None,
            description: Some(
                "Ethernet MAC address 0 high register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "maca0h",
                    description: Some(
                        "MAC address0 high",
                    ),
                    bit_offset: 0,
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mo",
                    description: Some(
                        "Always 1",
                    ),
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ptptslur",
            extends: None,
            description: Some(
                "Ethernet PTP time stamp low update register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tsuss",
                    description: Some(
                        "TSUSS",
                    ),
                    bit_offset: 0,
                    bit_size: 31,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tsupns",
                    description: Some(
                        "TSUPNS",
                    ),
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ptptssr",
            extends: None,
            description: Some(
                "Ethernet PTP time stamp status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tsso",
                    description: Some(
                        "TSSO",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tsttr",
                    description: Some(
                        "TSSO",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Maca2lr",
            extends: None,
            description: Some(
                "Ethernet MAC address 2 low register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "maca2l",
                    description: Some(
                        "MACA2L",
                    ),
                    bit_offset: 0,
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dmachtbar",
            extends: None,
            description: Some(
                "Ethernet DMA current host transmit buffer address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "htbap",
                    description: Some(
                        "Host transmit buffer address pointer",
                    ),
                    bit_offset: 0,
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Maca2hr",
            extends: None,
            description: Some(
                "Ethernet MAC address 2 high register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "maca2h",
                    description: Some(
                        "MAC2AH",
                    ),
                    bit_offset: 0,
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mbc",
                    description: Some(
                        "MBC",
                    ),
                    bit_offset: 24,
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sa",
                    description: Some(
                        "SA",
                    ),
                    bit_offset: 30,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "MacahrSa",
                    ),
                },
                Field {
                    name: "ae",
                    description: Some(
                        "AE",
                    ),
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "MacahrAe",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Maca3lr",
            extends: None,
            description: Some(
                "Ethernet MAC address 3 low register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "maca3l",
                    description: Some(
                        "MBCA3L",
                    ),
                    bit_offset: 0,
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ptptsar",
            extends: None,
            description: Some(
                "Ethernet PTP time stamp addend register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tsa",
                    description: Some(
                        "TSA",
                    ),
                    bit_offset: 0,
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Macmiidr",
            extends: None,
            description: Some(
                "Ethernet MAC MII data register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "md",
                    description: Some(
                        "MII data read from/written to the PHY",
                    ),
                    bit_offset: 0,
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dmasr",
            extends: None,
            description: Some(
                "Ethernet DMA status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ts",
                    description: Some(
                        "Transmit status",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tpss",
                    description: Some(
                        "Transmit process stopped status",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tbus",
                    description: Some(
                        "Transmit buffer unavailable status",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tjts",
                    description: Some(
                        "Transmit jabber timeout status",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ros",
                    description: Some(
                        "Receive overflow status",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tus",
                    description: Some(
                        "Transmit underflow status",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rs",
                    description: Some(
                        "Receive status",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rbus",
                    description: Some(
                        "Receive buffer unavailable status",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rpss",
                    description: Some(
                        "Receive process stopped status",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pwts",
                    description: Some(
                        "PWTS",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ets",
                    description: Some(
                        "Early transmit status",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fbes",
                    description: Some(
                        "Fatal bus error status",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ers",
                    description: Some(
                        "Early receive status",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ais",
                    description: Some(
                        "Abnormal interrupt summary",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nis",
                    description: Some(
                        "Normal interrupt summary",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rps",
                    description: Some(
                        "Receive process state",
                    ),
                    bit_offset: 17,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Rps",
                    ),
                },
                Field {
                    name: "tps",
                    description: Some(
                        "Transmit process state",
                    ),
                    bit_offset: 20,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Tps",
                    ),
                },
                Field {
                    name: "ebs",
                    description: Some(
                        "Error bits status",
                    ),
                    bit_offset: 23,
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mmcs",
                    description: Some(
                        "MMC status",
                    ),
                    bit_offset: 27,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pmts",
                    description: Some(
                        "PMT status",
                    ),
                    bit_offset: 28,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tsts",
                    description: Some(
                        "Time stamp trigger status",
                    ),
                    bit_offset: 29,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Macdbgr",
            extends: None,
            description: Some(
                "Ethernet MAC debug register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mmrpea",
                    description: Some(
                        "MAC MII receive protocol engine active",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "msfrwcs",
                    description: Some(
                        "MAC small FIFO read/write controllers status",
                    ),
                    bit_offset: 1,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rfwra",
                    description: Some(
                        "Rx FIFO write controller active",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rfrcs",
                    description: Some(
                        "Rx FIFO read controller status",
                    ),
                    bit_offset: 5,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rffl",
                    description: Some(
                        "Rx FIFO fill level",
                    ),
                    bit_offset: 8,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mmtea",
                    description: Some(
                        "MAC MII transmit engine active",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mtfcs",
                    description: Some(
                        "MAC transmit frame controller status",
                    ),
                    bit_offset: 17,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mtp",
                    description: Some(
                        "MAC transmitter in pause",
                    ),
                    bit_offset: 19,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tfrs",
                    description: Some(
                        "Tx FIFO read status",
                    ),
                    bit_offset: 20,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tfwa",
                    description: Some(
                        "Tx FIFO write active",
                    ),
                    bit_offset: 22,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tfne",
                    description: Some(
                        "Tx FIFO not empty",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tff",
                    description: Some(
                        "Tx FIFO full",
                    ),
                    bit_offset: 25,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ptptthr",
            extends: None,
            description: Some(
                "Ethernet PTP target time high register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ttsh",
                    description: Some(
                        "0",
                    ),
                    bit_offset: 0,
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ptpttlr",
            extends: None,
            description: Some(
                "Ethernet PTP target time low register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ttsl",
                    description: Some(
                        "TTSL",
                    ),
                    bit_offset: 0,
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dmamfbocr",
            extends: None,
            description: Some(
                "Ethernet DMA missed frame and buffer overflow counter register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mfc",
                    description: Some(
                        "Missed frames by the controller",
                    ),
                    bit_offset: 0,
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "omfc",
                    description: Some(
                        "Overflow bit for missed frame counter",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mfa",
                    description: Some(
                        "Missed frames by the application",
                    ),
                    bit_offset: 17,
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ofoc",
                    description: Some(
                        "Overflow bit for FIFO overflow counter",
                    ),
                    bit_offset: 28,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ptpppscr",
            extends: None,
            description: Some(
                "Ethernet PTP PPS control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tsso",
                    description: Some(
                        "TSSO",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tsttr",
                    description: Some(
                        "TSTTR",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Mmcrimr",
            extends: None,
            description: Some(
                "Ethernet MMC receive interrupt mask register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rfcem",
                    description: Some(
                        "Received frame CRC error mask",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Rfcem",
                    ),
                },
                Field {
                    name: "rfaem",
                    description: Some(
                        "Received frames alignment error mask",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Rfaem",
                    ),
                },
                Field {
                    name: "rgufm",
                    description: Some(
                        "Received good Unicast frames mask",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Rgufm",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Dmatdlar",
            extends: None,
            description: Some(
                "Ethernet DMA transmit descriptor list address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "stl",
                    description: Some(
                        "Start of transmit list",
                    ),
                    bit_offset: 0,
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Dtcefd",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "Drop frames with errors only in the receive checksum offload engine",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "Do not drop frames that only have errors in the receive checksum offload engine",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "MacahrAe",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "Address filters ignore this address",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "Address filters use this address",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Gu",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "Normal operation",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "Any unicast packet filtered by the MAC address recognition may be a wakeup frame",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pbl",
            description: None,
            bit_size: 6,
            variants: &[
                EnumVariant {
                    name: "PBL1",
                    description: Some(
                        "Maximum of 1 beat per DMA transaction",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PBL2",
                    description: Some(
                        "Maximum of 2 beats per DMA transaction",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "PBL4",
                    description: Some(
                        "Maximum of 4 beats per DMA transaction",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "PBL8",
                    description: Some(
                        "Maximum of 8 beats per DMA transaction",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "PBL16",
                    description: Some(
                        "Maximum of 16 beats per DMA transaction",
                    ),
                    value: 16,
                },
                EnumVariant {
                    name: "PBL32",
                    description: Some(
                        "Maximum of 32 beats per DMA transaction",
                    ),
                    value: 32,
                },
            ],
        },
        Enum {
            name: "Lm",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NORMAL",
                    description: Some(
                        "Normal mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LOOPBACK",
                    description: Some(
                        "MAC operates in loopback mode at the MII",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pcf",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PREVENTALL",
                    description: Some(
                        "MAC prevents all control frames from reaching the application",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FORWARDALLEXCEPTPAUSE",
                    description: Some(
                        "MAC forwards all control frames to application except Pause",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "FORWARDALL",
                    description: Some(
                        "MAC forwards all control frames to application even if they fail the address filter",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "FORWARDALLFILTERED",
                    description: Some(
                        "MAC forwards control frames that pass the address filter",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Usp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "COMBINED",
                    description: Some(
                        "PBL value used for both Rx and Tx DMA",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SEPARATE",
                    description: Some(
                        "RxDMA uses RDP value, TxDMA uses PBL value",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Fb",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "VARIABLE",
                    description: Some(
                        "AHB uses SINGLE and INCR burst transfers",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FIXED",
                    description: Some(
                        "AHB uses only fixed burst transfers",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Fcb",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLEBACKPRESSURE",
                    description: Some(
                        "In half duplex only, deasserts back pressure",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PAUSEORBACKPRESSURE",
                    description: Some(
                        "In full duplex, initiate a Pause control frame. In half duplex, assert back pressure",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "MacahrSa",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DESTINATION",
                    description: Some(
                        "This address is used for comparison with DA fields of the received frame",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SOURCE",
                    description: Some(
                        "This address is used for comparison with SA fields of received frames",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Hu",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "PERFECT",
                    description: Some(
                        "MAC performs a perfect destination address filtering for unicast frames",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HASH",
                    description: Some(
                        "MAC performs destination address filtering of received unicast frames according to the hash table",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Mcf",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "UNFROZEN",
                    description: Some(
                        "All MMC counters update normally",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FROZEN",
                    description: Some(
                        "All MMC counters frozen to their current value",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Csr",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "Counters roll over to zero after reaching the maximum value",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "Counters do not roll over to zero after reaching the maximum value",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "St",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "STOPPED",
                    description: Some(
                        "Transmission is placed in the Stopped state",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "STARTED",
                    description: Some(
                        "Transmission is placed in Running state",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Rod",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "MAC receives all packets from PHY while transmitting",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "MAC disables reception of frames in half-duplex mode",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Tsf",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CUTTHROUGH",
                    description: Some(
                        "Transmission starts when the frame size in the Tx FIFO exceeds TTC threshold",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "STOREFORWARD",
                    description: Some(
                        "Transmission starts when a full frame is in the Tx FIFO",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Vlantc",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "VLANTC16",
                    description: Some(
                        "Full 16 bit VLAN identifiers are used for comparison and filtering",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "VLANTC12",
                    description: Some(
                        "12 bit VLAN identifies are used for comparison and filtering",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Fef",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DROP",
                    description: Some(
                        "Rx FIFO drops frames with error status",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FORWARD",
                    description: Some(
                        "All frames except runt error frames are forwarded to the DMA",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Tgfm",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "UNMASKED",
                    description: Some(
                        "Transmitted-good counter half-full interrupt enabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MASKED",
                    description: Some(
                        "Transmitted-good counter half-full interrupt disabled",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Rgufm",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "UNMASKED",
                    description: Some(
                        "Received-good-unicast counter half-full interrupt enabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MASKED",
                    description: Some(
                        "Received-good-unicast counter half-full interrupt disabled",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Rtc",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "RTC64",
                    description: Some(
                        "64 bytes",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RTC32",
                    description: Some(
                        "32 bytes",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "RTC96",
                    description: Some(
                        "96 bytes",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "RTC128",
                    description: Some(
                        "128 bytes",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "CounterReset",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "RESET",
                    description: Some(
                        "Reset all counters. Cleared automatically",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Hpf",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "HASHONLY",
                    description: Some(
                        "If HM or HU is set, only frames that match the Hash filter are passed",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HASHORPERFECT",
                    description: Some(
                        "If HM or HU is set, frames that match either the perfect filter or the hash filter are passed",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Rd",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "MAC attempts retries based on the settings of BL",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "MAC attempts only 1 transmission",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Wfe",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "No power management event generated due to wakeup frame reception",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "Enable generation of a power management event due to wakeup frame reception",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Zqpd",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "Normal operation with automatic zero-quanta pause control frame generation",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "Automatic generation of zero-quanta pause control frames is disabled",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Apcs",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "MAC passes all incoming frames unmodified",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "STRIP",
                    description: Some(
                        "MAC strips the Pad/FCS field on incoming frames only for lengths less than or equal to 1500 bytes",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "PriorityRxOverTx",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "ONETOONE",
                    description: Some(
                        "RxDMA priority over TxDMA is 1:1",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "TWOTOONE",
                    description: Some(
                        "RxDMA priority over TxDMA is 2:1",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "THREETOONE",
                    description: Some(
                        "RxDMA priority over TxDMA is 3:1",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "FOURTOONE",
                    description: Some(
                        "RxDMA priority over TxDMA is 4:1",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Tfce",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "In full duplex, flow control is disabled. In half duplex, back pressure is disabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "In full duplex, flow control is enabled. In half duplex, back pressure is enabled",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "DmaomrSr",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "STOPPED",
                    description: Some(
                        "Reception is stopped after transfer of the current frame",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "STARTED",
                    description: Some(
                        "Reception is placed in the Running state",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Wffrpr",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "RESET",
                    description: Some(
                        "Reset wakeup frame filter register point to 0b000. Automatically cleared",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Mpe",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "No power management event generated due to Magic Packet reception",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "Enable generation of a power management event due to Magic Packet reception",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Rfcem",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "UNMASKED",
                    description: Some(
                        "Received-crc-error counter half-full interrupt enabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MASKED",
                    description: Some(
                        "Received-crc-error counter half-full interrupt disabled",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ra",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "MAC receiver passes on to the application only those frames that have passed the SA/DA address file",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "MAC receiver passes oll received frames on to the application",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pm",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "Normal address filtering",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "Address filters pass all incoming frames regardless of their destination or source address",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Wd",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "Watchdog enabled, receive frames limited to 2048 bytes",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "Watchdog disabled, receive frames may be up to to 16384 bytes",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Tstim",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "UNMASKED",
                    description: Some(
                        "Time stamp interrupt generation enabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MASKED",
                    description: Some(
                        "Time stamp interrupt generation disabled",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Fpm",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "X1",
                    description: Some(
                        "PBL values used as-is",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "X4",
                    description: Some(
                        "PBL values multiplied by 4",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Hm",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "PERFECT",
                    description: Some(
                        "MAC performs a perfect destination address filtering for multicast frames",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HASH",
                    description: Some(
                        "MAC performs destination address filtering of received multicast frames according to the hash table",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Rfce",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "Pause frames are not decoded",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "MAC decodes received Pause frames and disables its transmitted for a specified time",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ftf",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "FLUSH",
                    description: Some(
                        "Transmit FIFO controller logic is reset to its default values. Cleared automatically",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Upfd",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "MAC detects only a Pause frame with the multicast address specified in the 802.3x standard",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "MAC additionally detects Pause frames with the station's unicast address",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ror",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "MMC counters do not reset on read",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "MMC counters reset to zero after read",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Da",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ROUNDROBIN",
                    description: Some(
                        "Round-robin with Rx:Tx priority given by PM",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RXPRIORITY",
                    description: Some(
                        "Rx has priority over Tx",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Saf",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "Source address ignored",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "MAC drops frames that fail the source address filter",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Csd",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "Errors generated due to loss of carrier",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "No error generated due to loss of carrier",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Rps",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "STOPPED",
                    description: Some(
                        "Stopped, reset or Stop Receive command issued",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RUNNINGFETCHING",
                    description: Some(
                        "Running, fetching receive transfer descriptor",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "RUNNINGWAITING",
                    description: Some(
                        "Running, waiting for receive packet",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "SUSPENDED",
                    description: Some(
                        "Suspended, receive descriptor unavailable",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "RUNNINGWRITING",
                    description: Some(
                        "Running, writing data to host memory buffer",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Dm",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "HALFDUPLEX",
                    description: Some(
                        "MAC operates in half-duplex mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FULLDUPLEX",
                    description: Some(
                        "MAC operates in full-duplex mode",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Bfd",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "Address filters pass all received broadcast frames",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "Address filters filter all incoming broadcast frames",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Dc",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "MAC defers until CRS signal goes inactive",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "Deferral check function enabled",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Daif",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NORMAL",
                    description: Some(
                        "Normal filtering of frames",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "INVERT",
                    description: Some(
                        "Address check block operates in inverse filtering mode for the DA address comparison",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Rpd",
            description: None,
            bit_size: 32,
            variants: &[
                EnumVariant {
                    name: "POLL",
                    description: Some(
                        "Poll the receive descriptor list",
                    ),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Rdp",
            description: None,
            bit_size: 6,
            variants: &[
                EnumVariant {
                    name: "RDP1",
                    description: Some(
                        "1 beat per RxDMA transaction",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "RDP2",
                    description: Some(
                        "2 beats per RxDMA transaction",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "RDP4",
                    description: Some(
                        "4 beats per RxDMA transaction",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "RDP8",
                    description: Some(
                        "8 beats per RxDMA transaction",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "RDP16",
                    description: Some(
                        "16 beats per RxDMA transaction",
                    ),
                    value: 16,
                },
                EnumVariant {
                    name: "RDP32",
                    description: Some(
                        "32 beats per RxDMA transaction",
                    ),
                    value: 32,
                },
            ],
        },
        Enum {
            name: "Ipco",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "IPv4 checksum offload disabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "OFFLOAD",
                    description: Some(
                        "IPv4 checksums are checked in received frames",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pam",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "Filtering of multicast frames depends on HM",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "All received frames with a multicast destination address are passed",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Fes",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "FES10",
                    description: Some(
                        "10 Mbit/s",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FES100",
                    description: Some(
                        "100 Mbit/s",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Bl",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "BL10",
                    description: Some(
                        "For retransmission n, wait up to 2^min(n, 10) time slots",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BL8",
                    description: Some(
                        "For retransmission n, wait up to 2^min(n, 8) time slots",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "BL4",
                    description: Some(
                        "For retransmission n, wait up to 2^min(n, 4) time slots",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "BL1",
                    description: Some(
                        "For retransmission n, wait up to 2^min(n, 1) time slots",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Aab",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "UNALIGNED",
                    description: Some(
                        "Bursts are not aligned",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ALIGNED",
                    description: Some(
                        "Align bursts to start address LS bits. First burst alignment depends on FB bit",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Mw",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "READ",
                    description: Some(
                        "Read operation",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "WRITE",
                    description: Some(
                        "Write operation",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Tpd",
            description: None,
            bit_size: 32,
            variants: &[
                EnumVariant {
                    name: "POLL",
                    description: Some(
                        "Poll the transmit descriptor list",
                    ),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Rsf",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CUTTHROUGH",
                    description: Some(
                        "Rx FIFO operates in cut-through mode, subject to RTC bits",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "STOREFORWARD",
                    description: Some(
                        "Frames are read from Rx FIFO after complete frame has been written",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Plt",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PLT4",
                    description: Some(
                        "Pause time minus 4 slot times",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLT28",
                    description: Some(
                        "Pause time minus 28 slot times",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLT144",
                    description: Some(
                        "Pause time minus 144 slot times",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "PLT256",
                    description: Some(
                        "Pause time minus 256 slot times",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Ttc",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "TTC64",
                    description: Some(
                        "64 bytes",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "TTC128",
                    description: Some(
                        "128 bytes",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "TTC192",
                    description: Some(
                        "192 bytes",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "TTC256",
                    description: Some(
                        "256 bytes",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "TTC40",
                    description: Some(
                        "40 bytes",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "TTC32",
                    description: Some(
                        "32 bytes",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "TTC24",
                    description: Some(
                        "24 bytes",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "TTC16",
                    description: Some(
                        "16 bytes",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Cr",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "CR_60_100",
                    description: Some(
                        "60-100MHz HCLK/42",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CR_100_150",
                    description: Some(
                        "100-150 MHz HCLK/62",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "CR_20_35",
                    description: Some(
                        "20-35MHz HCLK/16",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "CR_35_60",
                    description: Some(
                        "35-60MHz HCLK/16",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "CR_150_168",
                    description: Some(
                        "150-168MHz HCLK/102",
                    ),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "Jd",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "Jabber enabled, transmit frames up to 2048 bytes",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "Jabber disabled, transmit frames up to 16384 bytes",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pd",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "All received frames will be dropped. Cleared automatically when a magic packet or wakeup frame is received",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Rfaem",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "UNMASKED",
                    description: Some(
                        "Received-alignment-error counter half-full interrupt enabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MASKED",
                    description: Some(
                        "Received-alignment-error counter half-full interrupt disabled",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Tps",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "STOPPED",
                    description: Some(
                        "Stopped, Reset or Stop Transmit command issued",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RUNNINGFETCHING",
                    description: Some(
                        "Running, fetching transmit transfer descriptor",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "RUNNINGWAITING",
                    description: Some(
                        "Running, waiting for status",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "RUNNINGREADING",
                    description: Some(
                        "Running, reading data from host memory buffer",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "SUSPENDED",
                    description: Some(
                        "Suspended, transmit descriptor unavailable or transmit buffer underflow",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "RUNNING",
                    description: Some(
                        "Running, closing transmit descriptor",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Ifg",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "IFG96",
                    description: Some(
                        "96 bit times",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "IFG88",
                    description: Some(
                        "88 bit times",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "IFG80",
                    description: Some(
                        "80 bit times",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "IFG72",
                    description: Some(
                        "72 bit times",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "IFG64",
                    description: Some(
                        "64 bit times",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "IFG56",
                    description: Some(
                        "56 bit times",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "IFG48",
                    description: Some(
                        "48 bit times",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "IFG40",
                    description: Some(
                        "40 bit times",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Saif",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NORMAL",
                    description: Some(
                        "Source address filter operates normally",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "INVERT",
                    description: Some(
                        "Source address filter operation inverted",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Tgfscm",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "UNMASKED",
                    description: Some(
                        "Transmitted-good-single-collision half-full interrupt enabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MASKED",
                    description: Some(
                        "Transmitted-good-single-collision half-full interrupt disabled",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "MbProgress",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "BUSY",
                    description: Some(
                        "This bit is set to 1 by the application to indicate that a read or write access is in progress",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Fugf",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DROP",
                    description: Some(
                        "Rx FIFO drops all frames of less than 64 bytes",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FORWARD",
                    description: Some(
                        "Rx FIFO forwards undersized frames",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pmtim",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "UNMASKED",
                    description: Some(
                        "PMT Status interrupt generation enabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MASKED",
                    description: Some(
                        "PMT Status interrupt generation disabled",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Tgfmscm",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "UNMASKED",
                    description: Some(
                        "Transmitted-good-multiple-collision half-full interrupt enabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MASKED",
                    description: Some(
                        "Transmitted-good-multiple-collision half-full interrupt disabled",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
