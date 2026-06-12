include!("../metadata_0803.rs");
use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
pub static METADATA: Metadata = Metadata {
    name: "STM32MP153FA",
    family: "STM32MP1",
    line: "STM32MP153",
    memory: &[&[MemoryRegion {
        name: "SRAM",
        kind: MemoryRegionKind::Ram,
        address: 0x10000000,
        size: 262144,
        settings: None,
    }]],
    peripherals: PERIPHERALS,
    nvic_priority_bits: Some(4),
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
    pins: PINS,
};
