include!("../metadata_0720.rs");
pub const METADATA: Metadata = Metadata {
    name: "STM32U575QI",
    family: "STM32U5",
    line: "STM32U575/585",
    memory: &[
        MemoryRegion {
            name: "BANK_1",
            kind: MemoryRegionKind::Flash,
            address: 134217728,
            size: 2097152,
            settings: Some(FlashSettings {
                erase_size: 8192,
                write_size: 16,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "OTP",
            kind: MemoryRegionKind::Flash,
            address: 200933376,
            size: 512,
            settings: Some(FlashSettings {
                erase_size: 512,
                write_size: 4,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "SRAM",
            kind: MemoryRegionKind::Ram,
            address: 536870912,
            size: 786432,
            settings: None,
        },
        MemoryRegion {
            name: "SRAM2",
            kind: MemoryRegionKind::Ram,
            address: 537067520,
            size: 0,
            settings: None,
        },
    ],
    peripherals: PERIPHERALS,
    nvic_priority_bits: Some(4),
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
