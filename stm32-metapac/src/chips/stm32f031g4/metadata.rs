include!("../metadata_0016.rs");
pub const METADATA: Metadata = Metadata {
    name: "STM32F031G4",
    family: "STM32F0",
    line: "STM32F0x1",
    memory: &[
        MemoryRegion {
            name: "BANK_1",
            kind: MemoryRegionKind::Flash,
            address: 134217728,
            size: 16384,
            settings: Some(FlashSettings {
                erase_size: 1024,
                write_size: 4,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "SRAM",
            kind: MemoryRegionKind::Ram,
            address: 536870912,
            size: 4096,
            settings: None,
        },
    ],
    peripherals: PERIPHERALS,
    nvic_priority_bits: Some(2),
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
