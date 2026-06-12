include!("../metadata_0804.rs");
use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
pub static METADATA: Metadata = Metadata {
    name: "STM32MP157AA",
    family: "STM32MP1",
    line: "STM32MP157",
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
