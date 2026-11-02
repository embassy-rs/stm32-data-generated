include!("../metadata_0451.rs");
use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
pub static METADATA: Metadata = Metadata {
    name: "STM32G553RC",
    family: "STM32G5",
    line: "STM32G5x3",
    memory: &[&[
        MemoryRegion {
            name: "BANK_1",
            kind: MemoryRegionKind::Flash,
            address: 0x8000000,
            size: 131072,
            settings: Some(FlashSettings {
                erase_size: 8192,
                write_size: 16,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "BANK_2",
            kind: MemoryRegionKind::Flash,
            address: 0x8020000,
            size: 131072,
            settings: Some(FlashSettings {
                erase_size: 8192,
                write_size: 16,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "OTP",
            kind: MemoryRegionKind::Flash,
            address: 0x8fff000,
            size: 2048,
            settings: Some(FlashSettings {
                erase_size: 0,
                write_size: 16,
                erase_value: 255,
            }),
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
            size: 65536,
            settings: None,
        },
        MemoryRegion {
            name: "SRAM3",
            kind: MemoryRegionKind::Ram,
            address: 0x20018000,
            size: 16384,
            settings: None,
        },
    ]],
    peripherals: PERIPHERALS,
    nvic_priority_bits: Some(4),
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
    pins: PINS,
};
