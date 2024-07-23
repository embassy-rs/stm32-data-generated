include!("../metadata_0261.rs");
            use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
            pub static METADATA: Metadata = Metadata {
                name: "STM32F778AI",
                family: "STM32F7",
                line: "STM32F7x9",
                memory: &[
    MemoryRegion {
        name: "BANK_1_REGION_1",
        kind: MemoryRegionKind::Flash,
        address: 0x8000000,
        size: 131072,
        settings: Some(
            FlashSettings {
                erase_size: 32768,
                write_size: 16,
                erase_value: 255,
            },
        ),
    },
    MemoryRegion {
        name: "BANK_1_REGION_2",
        kind: MemoryRegionKind::Flash,
        address: 0x8020000,
        size: 131072,
        settings: Some(
            FlashSettings {
                erase_size: 131072,
                write_size: 16,
                erase_value: 255,
            },
        ),
    },
    MemoryRegion {
        name: "BANK_1_REGION_3",
        kind: MemoryRegionKind::Flash,
        address: 0x8040000,
        size: 1835008,
        settings: Some(
            FlashSettings {
                erase_size: 262144,
                write_size: 16,
                erase_value: 255,
            },
        ),
    },
    MemoryRegion {
        name: "DTCM",
        kind: MemoryRegionKind::Ram,
        address: 0x20000000,
        size: 131072,
        settings: None,
    },
    MemoryRegion {
        name: "SRAM",
        kind: MemoryRegionKind::Ram,
        address: 0x20020000,
        size: 393216,
        settings: None,
    },
],
                peripherals: PERIPHERALS,
                nvic_priority_bits: Some(4),
                interrupts: INTERRUPTS,
                dma_channels: DMA_CHANNELS,
            };