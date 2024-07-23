include!("../metadata_0723.rs");
            use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
            pub static METADATA: Metadata = Metadata {
                name: "STM32L552RE",
                family: "STM32L5",
                line: "STM32L5x2",
                memory: &[
    MemoryRegion {
        name: "BANK_1",
        kind: MemoryRegionKind::Flash,
        address: 0x8000000,
        size: 524288,
        settings: Some(
            FlashSettings {
                erase_size: 4096,
                write_size: 8,
                erase_value: 255,
            },
        ),
    },
    MemoryRegion {
        name: "SRAM",
        kind: MemoryRegionKind::Ram,
        address: 0x20000000,
        size: 262144,
        settings: None,
    },
],
                peripherals: PERIPHERALS,
                nvic_priority_bits: Some(3),
                interrupts: INTERRUPTS,
                dma_channels: DMA_CHANNELS,
            };