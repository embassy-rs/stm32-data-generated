include!("../metadata_0400.rs");
use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
pub static METADATA: Metadata = Metadata {
    name: "STM32H733VG",
    family: "STM32H7",
    line: "STM32H723/733",
    memory: &[
        MemoryRegion {
            name: "D1_ITCMRAM",
            kind: MemoryRegionKind::Ram,
            address: 0x0,
            size: 0,
            settings: None,
        },
        MemoryRegion {
            name: "BANK_1",
            kind: MemoryRegionKind::Flash,
            address: 0x8000000,
            size: 1048576,
            settings: Some(FlashSettings {
                erase_size: 131072,
                write_size: 32,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "D1_AXIFLASH",
            kind: MemoryRegionKind::Flash,
            address: 0x8000000,
            size: 0,
            settings: None,
        },
        MemoryRegion {
            name: "D1_AXIICP",
            kind: MemoryRegionKind::Flash,
            address: 0x1ff00000,
            size: 0,
            settings: None,
        },
        MemoryRegion {
            name: "D1_DTCMRAM",
            kind: MemoryRegionKind::Ram,
            address: 0x20000000,
            size: 0,
            settings: None,
        },
        MemoryRegion {
            name: "SRAM",
            kind: MemoryRegionKind::Ram,
            address: 0x24000000,
            size: 131072,
            settings: None,
        },
        MemoryRegion {
            name: "D3_SRAM",
            kind: MemoryRegionKind::Ram,
            address: 0x38000000,
            size: 0,
            settings: None,
        },
        MemoryRegion {
            name: "D3_BKPSRAM",
            kind: MemoryRegionKind::Ram,
            address: 0x38800000,
            size: 0,
            settings: None,
        },
    ],
    peripherals: PERIPHERALS,
    nvic_priority_bits: Some(4),
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
