include!("../metadata_0274.rs");
use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
pub static METADATA: Metadata = Metadata {
    name: "STM32F439ZI",
    family: "STM32F4",
    line: "STM32F429/439",
    memory: &[&[
        MemoryRegion {
            name: "BANK_1_REGION_1",
            kind: MemoryRegionKind::Flash,
            address: 0x8000000,
            size: 65536,
            settings: Some(FlashSettings {
                erase_size: 16384,
                write_size: 4,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "BANK_1_REGION_2",
            kind: MemoryRegionKind::Flash,
            address: 0x8010000,
            size: 65536,
            settings: Some(FlashSettings {
                erase_size: 65536,
                write_size: 4,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "BANK_1_REGION_3",
            kind: MemoryRegionKind::Flash,
            address: 0x8020000,
            size: 917504,
            settings: Some(FlashSettings {
                erase_size: 131072,
                write_size: 4,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "BANK_2_REGION_1",
            kind: MemoryRegionKind::Flash,
            address: 0x8100000,
            size: 65536,
            settings: Some(FlashSettings {
                erase_size: 16384,
                write_size: 4,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "BANK_2_REGION_2",
            kind: MemoryRegionKind::Flash,
            address: 0x8110000,
            size: 65536,
            settings: Some(FlashSettings {
                erase_size: 65536,
                write_size: 4,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "BANK_2_REGION_3",
            kind: MemoryRegionKind::Flash,
            address: 0x8120000,
            size: 917504,
            settings: Some(FlashSettings {
                erase_size: 131072,
                write_size: 4,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "CCMRAM",
            kind: MemoryRegionKind::Ram,
            address: 0x10000000,
            size: 65536,
            settings: None,
        },
        MemoryRegion {
            name: "OTP",
            kind: MemoryRegionKind::Flash,
            address: 0x1fff7800,
            size: 528,
            settings: Some(FlashSettings {
                erase_size: 0,
                write_size: 4,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "SRAM",
            kind: MemoryRegionKind::Ram,
            address: 0x20000000,
            size: 196608,
            settings: None,
        },
        MemoryRegion {
            name: "BKPSRAM",
            kind: MemoryRegionKind::Ram,
            address: 0x40024000,
            size: 4096,
            settings: None,
        },
    ]],
    peripherals: PERIPHERALS,
    nvic_priority_bits: Some(4),
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
    pins: PINS,
    power: Some(Power {
        dialect: PowerDialect::Classic,
        stopwuck: false,
        modes: &[
            PowerMode {
                kind: PowerModeKind::Stop0,
                selector: 0,
                min_svos: None,
                surviving_clocks: &["LSE", "LSI"],
                min_residency_us: 200,
                exit_latency_us: 50,
            },
            PowerMode {
                kind: PowerModeKind::Stop1,
                selector: 1,
                min_svos: None,
                surviving_clocks: &["LSE", "LSI"],
                min_residency_us: 600,
                exit_latency_us: 300,
            },
        ],
    }),
    adc_calibration: Some(AdcCalibration {
        vrefint: Some(VrefCalibration {
            vref_mv: 3300,
            cal_bits: 12,
        }),
        temp: Some(TempCalibration::TwoPoint(TwoPointTempCal {
            vref_mv: 3300,
            cal_bits: 12,
            cal1_temp_c: 30,
            cal2_temp_c: 110,
        })),
    }),
};
