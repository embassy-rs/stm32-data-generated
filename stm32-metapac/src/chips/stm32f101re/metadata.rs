include!("../metadata_0093.rs");
use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
pub static METADATA: Metadata = Metadata {
    name: "STM32F101RE",
    family: "STM32F1",
    line: "STM32F101",
    memory: &[&[
        MemoryRegion {
            name: "BANK_1",
            kind: MemoryRegionKind::Flash,
            address: 0x8000000,
            size: 524288,
            settings: Some(FlashSettings {
                erase_size: 2048,
                write_size: 4,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "SRAM",
            kind: MemoryRegionKind::Ram,
            address: 0x20000000,
            size: 49152,
            settings: None,
        },
    ]],
    peripherals: PERIPHERALS,
    nvic_priority_bits: Some(4),
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
    pins: PINS,
};
