include!("../metadata_0260.rs");
pub static METADATA: Metadata = Metadata {
    name: "STM32F777ZI",
    family: "STM32F7",
    line: "STM32F7x7",
    memory: &[
        MemoryRegion {
            name: "BANK_1_REGION_1",
            kind: MemoryRegionKind::Flash,
            address: 134217728,
            size: 131072,
            settings: Some(FlashSettings {
                erase_size: 32768,
                write_size: 32,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "BANK_1_REGION_2",
            kind: MemoryRegionKind::Flash,
            address: 134348800,
            size: 131072,
            settings: Some(FlashSettings {
                erase_size: 131072,
                write_size: 32,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "BANK_1_REGION_3",
            kind: MemoryRegionKind::Flash,
            address: 134479872,
            size: 1835008,
            settings: Some(FlashSettings {
                erase_size: 262144,
                write_size: 32,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "OTP",
            kind: MemoryRegionKind::Flash,
            address: 535883776,
            size: 1152,
            settings: Some(FlashSettings {
                erase_size: 1152,
                write_size: 4,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "SRAM",
            kind: MemoryRegionKind::Ram,
            address: 537001984,
            size: 393216,
            settings: None,
        },
        MemoryRegion {
            name: "SRAM2",
            kind: MemoryRegionKind::Ram,
            address: 537378816,
            size: 0,
            settings: None,
        },
    ],
    peripherals: PERIPHERALS,
    nvic_priority_bits: Some(4),
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
