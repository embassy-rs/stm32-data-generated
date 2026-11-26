include!("../metadata_0040.rs");
use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
pub static METADATA: Metadata = Metadata {
    name: "STM32C551CC",
    family: "STM32C5",
    line: "STM32C55x/562",
    memory: &[
        &[
            MemoryRegion {
                name: "BANK_1",
                kind: MemoryRegionKind::Flash,
                address: 0x8000000,
                size: 262144,
                settings: Some(FlashSettings {
                    erase_size: 8192,
                    write_size: 16,
                    erase_value: 255,
                }),
            },
            MemoryRegion {
                name: "SRAM1",
                kind: MemoryRegionKind::Ram,
                address: 0x20000000,
                size: 65536,
                settings: None,
            },
            MemoryRegion {
                name: "SRAM2",
                kind: MemoryRegionKind::Ram,
                address: 0x20010000,
                size: 65536,
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
                name: "SRAM1",
                kind: MemoryRegionKind::Ram,
                address: 0x20000000,
                size: 65536,
                settings: None,
            },
            MemoryRegion {
                name: "SRAM2",
                kind: MemoryRegionKind::Ram,
                address: 0x20010000,
                size: 65536,
                settings: None,
            },
        ],
    ],
    peripherals: PERIPHERALS,
    nvic_priority_bits: Some(4),
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
    pins: PINS,
    power: Some(Power {
        dialect: PowerDialect::PmcrSvos,
        stopwuck: true,
        modes: &[
            PowerMode {
                kind: PowerModeKind::Stop0,
                selector: 0,
                min_svos: None,
                surviving_clocks: &["HSI", "LSE", "LSI"],
                min_residency_us: 100,
                exit_latency_us: 10,
            },
            PowerMode {
                kind: PowerModeKind::Stop1,
                selector: 1,
                min_svos: None,
                surviving_clocks: &["LSE", "LSI"],
                min_residency_us: 500,
                exit_latency_us: 43,
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
            cal2_temp_c: 130,
        })),
    }),
};
