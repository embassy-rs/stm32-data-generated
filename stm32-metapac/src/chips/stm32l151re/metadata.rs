include!("../metadata_0607.rs");
use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
pub static METADATA: Metadata = Metadata {
    name: "STM32L151RE",
    family: "STM32L1",
    line: "STM32L151/152",
    memory: &[&[
        MemoryRegion {
            name: "BANK_1",
            kind: MemoryRegionKind::Flash,
            address: 0x8000000,
            size: 262144,
            settings: Some(FlashSettings {
                erase_size: 256,
                write_size: 4,
                erase_value: 0,
            }),
        },
        MemoryRegion {
            name: "BANK_2",
            kind: MemoryRegionKind::Flash,
            address: 0x8040000,
            size: 262144,
            settings: Some(FlashSettings {
                erase_size: 256,
                write_size: 4,
                erase_value: 0,
            }),
        },
        MemoryRegion {
            name: "EEPROM_BANK_1",
            kind: MemoryRegionKind::Eeprom,
            address: 0x8080000,
            size: 8192,
            settings: None,
        },
        MemoryRegion {
            name: "EEPROM_BANK_2",
            kind: MemoryRegionKind::Eeprom,
            address: 0x8082000,
            size: 8192,
            settings: None,
        },
        MemoryRegion {
            name: "SRAM",
            kind: MemoryRegionKind::Ram,
            address: 0x20000000,
            size: 81920,
            settings: None,
        },
    ]],
    peripherals: PERIPHERALS,
    nvic_priority_bits: Some(4),
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
    pins: PINS,
};
