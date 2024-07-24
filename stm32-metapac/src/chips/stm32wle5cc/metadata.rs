include!("../metadata_0820.rs");
            use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
            pub static METADATA: Metadata = Metadata {
                name: "STM32WLE5CC",
                family: "STM32WL",
                line: "STM32WLEx",
                memory: &[
    MemoryRegion {
        name: "BANK_1",
        kind: MemoryRegionKind::Flash,
        address: 0x8000000,
        size: 262144,
        settings: Some(
            FlashSettings {
                erase_size: 2048,
                write_size: 8,
                erase_value: 255,
            },
        ),
    },
    MemoryRegion {
        name: "SRAM1",
        kind: MemoryRegionKind::Ram,
        address: 0x20000000,
        size: 32768,
        settings: None,
    },
    MemoryRegion {
        name: "SRAM2",
        kind: MemoryRegionKind::Ram,
        address: 0x20008000,
        size: 32768,
        settings: None,
    },
],
                peripherals: PERIPHERALS,
                nvic_priority_bits: Some(4),
                interrupts: INTERRUPTS,
                dma_channels: DMA_CHANNELS,
            };