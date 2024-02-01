include!("../metadata_0682.rs");
pub static METADATA: Metadata = Metadata {
    name: "STM32L552QC",
    family: "STM32L5",
    line: "STM32L5x2",
    memory: &[
        MemoryRegion {
            name: "BANK_1",
            kind: MemoryRegionKind::Flash,
            address: 134217728,
            size: 262144,
            settings: Some(FlashSettings {
                erase_size: 4096,
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
            address: 537067520,
            size: 0,
            settings: None,
        },
    ],
    peripherals: PERIPHERALS,
    nvic_priority_bits: Some(3),
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
