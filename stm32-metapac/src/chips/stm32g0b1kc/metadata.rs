include!("../metadata_0327.rs");
use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
pub static METADATA: Metadata = Metadata {
    name: "STM32G0B1KC",
    family: "STM32G0",
    line: "STM32G0x1",
    memory: &[
        &[
            MemoryRegion {
                name: "BANK_1",
                kind: MemoryRegionKind::Flash,
                address: 0x8000000,
                size: 262144,
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
                size: 147456,
                settings: None,
            },
        ],
        &[
            MemoryRegion {
                name: "BANK_1",
                kind: MemoryRegionKind::Flash,
                address: 0x8000000,
                size: 131072,
                settings: Some(FlashSettings {
                    erase_size: 2048,
                    write_size: 8,
                    erase_value: 255,
                }),
            },
            MemoryRegion {
                name: "BANK_2",
                kind: MemoryRegionKind::Flash,
                address: 0x8020000,
                size: 131072,
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
                size: 147456,
                settings: None,
            },
        ],
    ],
    peripherals: PERIPHERALS,
    nvic_priority_bits: Some(2),
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
    pins: PINS,
};
