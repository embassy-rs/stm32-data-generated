include!("../metadata_0936.rs");
use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
pub static METADATA: Metadata = Metadata {
    name: "STM32U356VC",
    family: "STM32U3",
    line: "STM32U3x6",
    memory: &[&[
        MemoryRegion {
            name: "BANK_1",
            kind: MemoryRegionKind::Flash,
            address: 0x8000000,
            size: 262144,
            settings: Some(FlashSettings {
                erase_size: 4096,
                write_size: 8,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "OTP",
            kind: MemoryRegionKind::Flash,
            address: 0xbfa0000,
            size: 512,
            settings: Some(FlashSettings {
                erase_size: 0,
                write_size: 8,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "SRAM",
            kind: MemoryRegionKind::Ram,
            address: 0x20000000,
            size: 131072,
            settings: None,
        },
        MemoryRegion {
            name: "SRAM2",
            kind: MemoryRegionKind::Ram,
            address: 0x20020000,
            size: 65536,
            settings: None,
        },
    ]],
    peripherals: PERIPHERALS,
    nvic_priority_bits: Some(4),
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
    pins: PINS,
    power: Some(Power {
        dialect: PowerDialect::LpmsSrWuscr,
        stopwuck: true,
        modes: &[
            PowerMode {
                kind: PowerModeKind::Stop0,
                selector: 0,
                min_svos: None,
                surviving_clocks: &["HSI", "MSIK", "LSE", "LSI"],
                min_residency_us: 100,
                exit_latency_us: 23,
            },
            PowerMode {
                kind: PowerModeKind::Stop1,
                selector: 1,
                min_svos: None,
                surviving_clocks: &["HSI", "MSIK", "LSE", "LSI"],
                min_residency_us: 500,
                exit_latency_us: 37,
            },
            PowerMode {
                kind: PowerModeKind::Stop2,
                selector: 2,
                min_svos: None,
                surviving_clocks: &["HSI", "MSIK", "LSE", "LSI"],
                min_residency_us: 900,
                exit_latency_us: 58,
            },
            PowerMode {
                kind: PowerModeKind::Stop3,
                selector: 3,
                min_svos: None,
                surviving_clocks: &["LSE", "LSI"],
                min_residency_us: 200000,
                exit_latency_us: 130,
            },
        ],
    }),
    adc_calibration: Some(AdcCalibration {
        vrefint: Some(VrefCalibration {
            vref_mv: 3000,
            cal_bits: 12,
        }),
        temp: Some(TempCalibration::TwoPoint(TwoPointTempCal {
            vref_mv: 3000,
            cal_bits: 12,
            cal1_temp_c: 30,
            cal2_temp_c: 110,
        })),
    }),
};
