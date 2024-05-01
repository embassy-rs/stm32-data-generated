include!("../metadata_0756.rs");
            use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
            pub static METADATA: Metadata = Metadata {
                name: "STM32U545VE",
                family: "STM32U5",
                line: "STM32U5x5",
                memory: &[
    MemoryRegion {
        name: "BANK_1",
        kind: MemoryRegionKind::Flash,
        address: 0x8000000,
        size: 262144,
        settings: Some(
            FlashSettings {
                erase_size: 16384,
                write_size: 16,
                erase_value: 255,
            },
        ),
    },
    MemoryRegion {
        name: "BANK_2",
        kind: MemoryRegionKind::Flash,
        address: 0x8040000,
        size: 262144,
        settings: Some(
            FlashSettings {
                erase_size: 16384,
                write_size: 16,
                erase_value: 255,
            },
        ),
    },
    MemoryRegion {
        name: "SRAM",
        kind: MemoryRegionKind::Ram,
        address: 0x20000000,
        size: 196608,
        settings: None,
    },
    MemoryRegion {
        name: "SRAM2",
        kind: MemoryRegionKind::Ram,
        address: 0x20030000,
        size: 65536,
        settings: None,
    },
],
                peripherals: PERIPHERALS,
                nvic_priority_bits: Some(4),
                interrupts: INTERRUPTS,
                dma_channels: DMA_CHANNELS,
            };