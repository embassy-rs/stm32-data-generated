include!("../metadata_0642.rs");
pub static METADATA: Metadata = Metadata {
    name: "STM32L496ZE",
    family: "STM32L4",
    line: "STM32L4x6",
    memory: &[
        MemoryRegion {
            name: "BANK_1",
            kind: MemoryRegionKind::Flash,
            address: 134217728,
            size: 524288,
            settings: Some(FlashSettings {
                erase_size: 2048,
                write_size: 8,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "SRAM",
            kind: MemoryRegionKind::Ram,
            address: 536870912,
            size: 262144,
            settings: None,
        },
        MemoryRegion {
            name: "SRAM2",
            kind: MemoryRegionKind::Ram,
            address: 268435456,
            size: 0,
            settings: None,
        },
    ],
    peripherals: PERIPHERALS,
    nvic_priority_bits: Some(4),
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
