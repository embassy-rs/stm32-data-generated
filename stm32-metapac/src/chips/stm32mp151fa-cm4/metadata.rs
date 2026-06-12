include!("../metadata_0801.rs");
use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
pub static METADATA: Metadata = Metadata {
    name: "STM32MP151FA",
    family: "STM32MP1",
    line: "STM32MP151",
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
