include!("../metadata_0750.rs");
pub const METADATA: Metadata = Metadata {
    name: "STM32WB10CC",
    family: "STM32WB",
    line: "STM32WBx0 Value Line",
    memory: &[
        MemoryRegion {
            name: "BANK_1",
            kind: MemoryRegionKind::Flash,
            address: 134217728,
            size: 327680,
            settings: Some(FlashSettings {
                erase_size: 2048,
                write_size: 8,
                erase_value: 0,
            }),
        },
        MemoryRegion {
            name: "SRAM",
            kind: MemoryRegionKind::Ram,
            address: 536870912,
            size: 12288,
            settings: None,
        },
    ],
    peripherals: PERIPHERALS,
    nvic_priority_bits: Some(4),
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
