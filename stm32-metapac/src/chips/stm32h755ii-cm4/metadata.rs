include!("../metadata_0458.rs");
            use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
            pub static METADATA: Metadata = Metadata {
                name: "STM32H755II",
                family: "STM32H7",
                line: "STM32H745/755",
                memory: &[
    MemoryRegion {
        name: "ITCM",
        kind: MemoryRegionKind::Ram,
        address: 0x0,
        size: 65536,
        settings: None,
    },
    MemoryRegion {
        name: "BANK_1",
        kind: MemoryRegionKind::Flash,
        address: 0x8000000,
        size: 1048576,
        settings: Some(
            FlashSettings {
                erase_size: 131072,
                write_size: 32,
                erase_value: 255,
            },
        ),
    },
    MemoryRegion {
        name: "BANK_2",
        kind: MemoryRegionKind::Flash,
        address: 0x8100000,
        size: 1048576,
        settings: Some(
            FlashSettings {
                erase_size: 131072,
                write_size: 32,
                erase_value: 255,
            },
        ),
    },
    MemoryRegion {
        name: "RAM_D2",
        kind: MemoryRegionKind::Ram,
        address: 0x10000000,
        size: 294912,
        settings: None,
    },
    MemoryRegion {
        name: "RAM_D3",
        kind: MemoryRegionKind::Ram,
        address: 0x18000000,
        size: 65536,
        settings: None,
    },
    MemoryRegion {
        name: "DTCM",
        kind: MemoryRegionKind::Ram,
        address: 0x20000000,
        size: 131072,
        settings: None,
    },
    MemoryRegion {
        name: "RAM_D1",
        kind: MemoryRegionKind::Ram,
        address: 0x24000000,
        size: 524288,
        settings: None,
    },
],
                peripherals: PERIPHERALS,
                nvic_priority_bits: Some(4),
                interrupts: INTERRUPTS,
                dma_channels: DMA_CHANNELS,
            };