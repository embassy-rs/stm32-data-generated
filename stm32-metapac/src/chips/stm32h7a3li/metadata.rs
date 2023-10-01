include!("../metadata_0469.rs");
pub const METADATA: Metadata = Metadata {
    name: "STM32H7A3LI",
    family: "STM32H7",
    line: "STM32H7A3/7B3",
    memory: &[
        MemoryRegion {
            name: "BANK_1",
            kind: MemoryRegionKind::Flash,
            address: 134217728,
            size: 1048576,
            settings: Some(FlashSettings {
                erase_size: 8192,
                write_size: 32,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "BANK_2",
            kind: MemoryRegionKind::Flash,
            address: 135266304,
            size: 1048576,
            settings: Some(FlashSettings {
                erase_size: 8192,
                write_size: 32,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "OTP",
            kind: MemoryRegionKind::Flash,
            address: 150990848,
            size: 1024,
            settings: Some(FlashSettings {
                erase_size: 1024,
                write_size: 32,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "SRAM",
            kind: MemoryRegionKind::Ram,
            address: 603979776,
            size: 1048576,
            settings: None,
        },
    ],
    peripherals: PERIPHERALS,
    nvic_priority_bits: Some(4),
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
