include!("../metadata_0736.rs");
use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
pub static METADATA: Metadata = Metadata {
    name: "STM32U031R8",
    family: "STM32U0",
    line: "STM32U0x1",
    memory: &[
        MemoryRegion {
            name: "BANK_1",
            kind: MemoryRegionKind::Flash,
            address: 0x8000000,
            size: 65536,
            settings: Some(FlashSettings {
                erase_size: 2048,
                write_size: 8,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "SRAM",
            kind: MemoryRegionKind::Ram,
            address: 0x20000000,
            size: 12288,
            settings: None,
        },
    ],
    peripherals: PERIPHERALS,
    nvic_priority_bits: Some(2),
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
