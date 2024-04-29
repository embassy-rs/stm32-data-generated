include!("../metadata_0382.rs");
            use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
            pub static METADATA: Metadata = Metadata {
                name: "STM32H562II",
                family: "STM32H5",
                line: "STM32H562",
                memory: &[
    MemoryRegion {
        name: "BANK_1",
        kind: MemoryRegionKind::Flash,
        address: 0x8000000,
        size: 1048576,
        settings: Some(
            FlashSettings {
                erase_size: 8192,
                write_size: 16,
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
                erase_size: 8192,
                write_size: 16,
                erase_value: 255,
            },
        ),
    },
    MemoryRegion {
        name: "SRAM1",
        kind: MemoryRegionKind::Ram,
        address: 0x20000000,
        size: 262144,
        settings: None,
    },
    MemoryRegion {
        name: "SRAM2",
        kind: MemoryRegionKind::Ram,
        address: 0x20040000,
        size: 65536,
        settings: None,
    },
    MemoryRegion {
        name: "SRAM3",
        kind: MemoryRegionKind::Ram,
        address: 0x20050000,
        size: 327680,
        settings: None,
    },
],
                peripherals: PERIPHERALS,
                nvic_priority_bits: Some(4),
                interrupts: INTERRUPTS,
                dma_channels: DMA_CHANNELS,
            };